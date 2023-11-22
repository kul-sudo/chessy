// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use rand::seq::IteratorRandom;
use shakmaty::{
    fen::{Epd, Fen},
    CastlingMode, Chess, Color, EnPassantMode, Move, Position, Role,
};

static TREE_HEIGHT: i16 = 4; // It has to be either equal to or greater than 3

static STALEMATE_WEIGHT: i16 = 200;

// Piece weights
static KNIGHT_WEIGHT: i16 = 3;
static BISHOP_WEIGHT: i16 = 3;
static QUEEN_WEIGHT: i16 = 9;
static ROOK_WEIGHT: i16 = 5;
static PAWN_WEIGHT: i16 = 1;

struct Node {
    fen: String,
    layer_number: i16,
    bot_color: Color,
    bot_wants_stalemate: bool,
    enemy_wants_stalemate: bool,
    previous_move: Move,
    previous_weight: i16,
}

struct NodeProps {
    weight: i16,
    successors_number: i16,
    // next_successors_max_number: i16 // Unused right now
}

impl Node {
    fn get_weight(&self) -> NodeProps {
        let fen: Fen = self.fen.parse().unwrap();
        let chess: Chess = fen.clone().into_position(CastlingMode::Standard).unwrap();

        let turn = chess.turn();

        // Handling a checkmate
        if chess.is_checkmate() {
            return if self.bot_color == turn {
                NodeProps {
                    weight: i16::MIN,
                    successors_number: 0,
                }
            } else {
                NodeProps {
                    weight: i16::MAX,
                    successors_number: 0,
                }
            };
        }

        // Handling a stalemate
        if chess.is_stalemate() {
            if self.bot_color == turn {
                return if self.enemy_wants_stalemate {
                    NodeProps {
                        weight: STALEMATE_WEIGHT,
                        successors_number: 0,
                    }
                } else {
                    NodeProps {
                        weight: -STALEMATE_WEIGHT,
                        successors_number: 0,
                    }
                };
            } else {
                return if self.bot_wants_stalemate {
                    NodeProps {
                        weight: STALEMATE_WEIGHT,
                        successors_number: 0,
                    }
                } else {
                    NodeProps {
                        weight: -STALEMATE_WEIGHT,
                        successors_number: 0,
                    }
                };
            }
        }

        // Handling the other cases
        let mut current_weight = self.previous_weight;

        let is_capture = self.previous_move.is_capture();
        let is_promotion = self.previous_move.is_promotion();

        if is_capture || is_promotion {
            let coefficient = if turn == self.bot_color { -1 } else { 1 };

            if is_capture {
                let delta = match self.previous_move.capture().unwrap() {
                    Role::Knight => KNIGHT_WEIGHT,
                    Role::Bishop => BISHOP_WEIGHT,
                    Role::Queen => QUEEN_WEIGHT,
                    Role::Rook => ROOK_WEIGHT,
                    Role::Pawn => PAWN_WEIGHT,
                    Role::King => 0 // Artificially avoiding "_ => ()"
                };

                current_weight += delta * coefficient
            }

            if is_promotion {
                // The pawn always turns into a queen
                current_weight += (QUEEN_WEIGHT - PAWN_WEIGHT) * coefficient;
            }
        } else {
            current_weight = self.previous_weight;
        }

        if self.layer_number == TREE_HEIGHT {
            return NodeProps {
                weight: current_weight,
                successors_number: 0,
            };
        } else {
            let turn_for_bot = self.bot_color == turn;
            let mut result = if turn_for_bot { i16::MIN } else { i16::MAX };

            let legal_moves = &chess.legal_moves();

            for legal_move in legal_moves {
                let pos_after_move = chess.clone().play(&legal_move).unwrap();

                let node_props = (Node {
                    fen: Epd::from_position(pos_after_move, EnPassantMode::Legal).to_string(),
                    layer_number: self.layer_number + 1,
                    bot_color: self.bot_color,
                    bot_wants_stalemate: self.bot_wants_stalemate,
                    enemy_wants_stalemate: self.enemy_wants_stalemate,
                    previous_move: legal_move.clone(),
                    previous_weight: current_weight,
                })
                .get_weight();

                result = if turn_for_bot {
                    result.max(node_props.weight)
                } else {
                    result.min(node_props.weight)
                }
            }

            return NodeProps {
                weight: result,
                successors_number: legal_moves.len() as i16,
            };
        }
    }
}

fn get_weight_by_fen(fen: &str, bot_color: Color) -> i16 {
    // Calculating the weight for either black or white
    let only_pieces = fen.split_once(' ').unwrap().0;

    let mut weight_for_white: i16 = 0;

    for piece in only_pieces.chars() {
        match piece {
            'P' => weight_for_white += PAWN_WEIGHT,
            'R' => weight_for_white += ROOK_WEIGHT,
            'Q' => weight_for_white += QUEEN_WEIGHT,
            'B' => weight_for_white += BISHOP_WEIGHT,
            'N' => weight_for_white += KNIGHT_WEIGHT,
            'p' => weight_for_white -= PAWN_WEIGHT,
            'r' => weight_for_white -= ROOK_WEIGHT,
            'q' => weight_for_white -= QUEEN_WEIGHT,
            'b' => weight_for_white -= BISHOP_WEIGHT,
            'n' => weight_for_white -= KNIGHT_WEIGHT,
            _ => (),
        }
    }

    return if bot_color == Color::White {
        weight_for_white
    } else {
        -weight_for_white
    };
}

#[tauri::command(async, rename_all = "snake_case")]
async fn get_move(current_fen: String) -> String {
    let fen: Fen = current_fen.parse().unwrap();
    let chess: Chess = fen.clone().into_position(CastlingMode::Standard).unwrap();

    let bot_color = chess.turn();
    let weight_by_fen = get_weight_by_fen(&current_fen, bot_color);

    let legal_moves = chess.legal_moves();

    println!("{:?}", legal_moves);

    let mut move_props = HashMap::with_capacity(legal_moves.len());

    for legal_move in legal_moves {
        let pos_after_move = chess.clone().play(&legal_move).unwrap();

        move_props.insert(
            legal_move.clone(),
            (Node {
                fen: Epd::from_position(pos_after_move, EnPassantMode::Legal).to_string(),
                layer_number: 1,
                bot_color,
                bot_wants_stalemate: weight_by_fen <= -ROOK_WEIGHT,
                enemy_wants_stalemate: weight_by_fen >= ROOK_WEIGHT,
                previous_move: legal_move.clone(),
                previous_weight: weight_by_fen as i16,
            })
            .get_weight(),
        );
    }

    let max_weight_moves = move_props
        .iter()
        .max_by_key(|(_, value)| value.weight)
        .unwrap()
        .1
        .weight;

    move_props.retain(|_, move_weight| move_weight.weight == max_weight_moves); // Retaining the
                                                                                // moves with the best final weight

    let min_opponent_moves = move_props
        .iter()
        .min_by_key(|(_, value)| value.weight)
        .unwrap()
        .1
        .weight;

    move_props.retain(|_, move_weight| move_weight.weight == min_opponent_moves); // Retaining the moves with lowest number of moves the opponent can make next

    return move_props
        .keys()
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_move])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
