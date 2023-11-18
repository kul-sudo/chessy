// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use rand::seq::IteratorRandom;
use shakmaty::{
    fen::{Epd, Fen},
    CastlingMode, Chess, Color, EnPassantMode, Position
};

static TREE_HEIGHT: i16 = 3; // It has to be either equal to or greater than 3

static STALEMATE_WEIGHT: i16 = 200;

// Piece weights
static KNIGHT_WEIGHT: isize = 3;
static BISHOP_WEIGHT: isize = 3;
static QUEEN_WEIGHT: isize = 9;
static ROOK_WEIGHT: isize = 5;
static PAWN_WEIGHT: isize = 1;

struct Node {
    fen: String,
    layer_number: i16,
    bot_color: Color,
    bot_wants_stalemate: bool,
    enemy_wants_stalemate: bool,
}

#[derive(Debug)]
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
                        weight: 0 - STALEMATE_WEIGHT,
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
                        weight: 0 - STALEMATE_WEIGHT,
                        successors_number: 0,
                    }
                };
            }
        }

        // Handling the other cases
        if self.layer_number == TREE_HEIGHT {
            return NodeProps {
                weight: get_weight_by_fen(&self.fen, self.bot_color) as i16,
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

fn get_weight_by_fen(fen: &str, bot_color: Color) -> isize {
    // Calculating the weight for either black or white
    let only_pieces = &fen[..fen.chars().position(|c| c == ' ').unwrap()];

    let weight_for_white = only_pieces.matches("P").count() as isize * PAWN_WEIGHT
        + only_pieces.matches("R").count() as isize * ROOK_WEIGHT
        + only_pieces.matches("Q").count() as isize * QUEEN_WEIGHT
        + only_pieces.matches("B").count() as isize * BISHOP_WEIGHT
        + only_pieces.matches("N").count() as isize * KNIGHT_WEIGHT
        - only_pieces.matches("p").count() as isize * PAWN_WEIGHT
        - only_pieces.matches("r").count() as isize * ROOK_WEIGHT
        - only_pieces.matches("q").count() as isize * QUEEN_WEIGHT
        - only_pieces.matches("b").count() as isize * BISHOP_WEIGHT
        - only_pieces.matches("n").count() as isize * KNIGHT_WEIGHT;

    return if bot_color == Color::White {
        weight_for_white
    } else {
        0 - weight_for_white
    };
}

#[tauri::command(async, rename_all = "snake_case")]
async fn get_move(current_fen: String) -> String {
    let fen: Fen = current_fen.parse().unwrap();
    let chess: Chess = fen.clone().into_position(CastlingMode::Standard).unwrap();
    
    let bot_color = chess.turn();
    let legal_moves = chess.legal_moves();

    let mut move_props = HashMap::with_capacity(legal_moves.len());

    for legal_move in &legal_moves {
        let pos_after_move = chess.clone().play(&legal_move).unwrap();
        let weight_by_fen = get_weight_by_fen(&current_fen, bot_color);

        move_props.insert(
            legal_move,
            (Node {
                fen: Epd::from_position(pos_after_move, EnPassantMode::Legal).to_string(),
                layer_number: 1,
                bot_color,
                bot_wants_stalemate: weight_by_fen <= 0 - ROOK_WEIGHT,
                enemy_wants_stalemate: weight_by_fen >= ROOK_WEIGHT,
            })
            .get_weight(),
        );
    }


    let mut max_weight_moves = i16::MIN;
    for (_key, value) in &move_props {
        max_weight_moves = value.weight.max(max_weight_moves)
    }

    move_props.retain(|_, move_weight| move_weight.weight != max_weight_moves); // Retaining the
                                                                                // moves with the best final weight

    let mut min_opponent_moves = i16::MIN;
    for (_key, value) in &move_props {
        min_opponent_moves = value.weight.min(min_opponent_moves)
    }

    move_props.retain(|_, move_weight| move_weight.weight != min_opponent_moves); // Retaining the moves with lowest number of moves the opponent can make next

    return move_props.keys().choose(&mut rand::thread_rng()).unwrap().to_string();
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_move])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
