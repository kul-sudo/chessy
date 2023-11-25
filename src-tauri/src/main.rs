// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use rand::seq::IteratorRandom;
use shakmaty::{
    fen::{Epd, Fen},
    CastlingMode, Chess, Color, EnPassantMode, Move, Position, Role,
};

static TREE_HEIGHT: i16 = 4; // It has to be either equal to or greater than 3

static MAX_LEGAL_MOVES: i16 = 100;

// Piece weights
static PAWN_WEIGHT: i16 = 2 * MAX_LEGAL_MOVES + 1; // 1 * (2 * MAX_LEGAL_MOVES + 1)
static KNIGHT_WEIGHT: i16 = 3 * PAWN_WEIGHT;
static BISHOP_WEIGHT: i16 = 3 * PAWN_WEIGHT;
static QUEEN_WEIGHT: i16 = 9 * PAWN_WEIGHT;
static ROOK_WEIGHT: i16 = 5 * PAWN_WEIGHT;

static CHECKMATE_WEIGHT: i16 = i16::MAX;
static STALEMATE_WEIGHT: i16 = CHECKMATE_WEIGHT - 1;

/// Node struct.
struct Node {
    /// The FEN of the node.
    fen: String,

    /// The layer number the node is currently at.
    layer_number: i16,

    /// The color the bot is playing with.
    bot_color: Color,

    /// It's been experimentally determined that when the bot doesn't have a rook, it should want to get a stalemate.
    bot_wants_stalemate: bool,

    /// The move of the node edge (every egde connects 2 nodes).
    previous_move: Move,

    /// The weight of the predecessor node.
    previous_weight: i16,
}

impl Node {
    /// Get the weight of the current node (the final weight when both the bot and the opponent play in the best way possible);
    /// this weight may be adjusted according to the number of the legal moves that can me made by either the bot or the opponent.
    fn get_node_weight(self) -> i16 {
        // Create an instance of Chess with the current FEN
        let fen: Fen = self.fen.parse().unwrap();
        let chess: Chess = fen.clone().into_position(CastlingMode::Standard).unwrap();

        let turn = chess.turn();

        // Handle a possible checkmate
        if chess.is_checkmate() {
            // Return the worst or best weight depending on who the checkmate has been performed by
            return if self.bot_color == turn {
                -CHECKMATE_WEIGHT
            } else {
                CHECKMATE_WEIGHT
            };
        }

        // Handle a possible stalemate
        if chess.is_stalemate() {
            // Return the worst or best weight depending on whether the bot wants a stalemate;
            // however, a checkmate has a higher weight than a stalemate
            return if self.bot_wants_stalemate {
                STALEMATE_WEIGHT
            } else {
                -STALEMATE_WEIGHT
            };
        }

        // Handling the other cases (everything all the way down)

        // If there has been no pawn promotion or capture, there's no need to recalculate the weight
        let mut current_node_weight = self.previous_weight;

        let is_capture = self.previous_move.is_capture();
        let is_promotion = self.previous_move.is_promotion();

        if is_capture || is_promotion {
            // If there has either been a capture or promotion, an
            // adjustment is done

            let coefficient = if turn == self.bot_color { -1 } else { 1 }; // Defines whether a capture or promotion is good for the bot depending on the turn/color

            if is_capture {
                // Adjust the weight as a result of the captured piece
                current_node_weight +=
                    self.get_piece_weight(self.previous_move.capture().unwrap()) * coefficient
            }

            if is_promotion {
                // Adjust the weight as a result of the promoted pawn
                current_node_weight +=
                    (self.get_piece_weight(self.previous_move.promotion().unwrap()) - PAWN_WEIGHT)
                        * coefficient;
            }
        } else {
            current_node_weight = self.previous_weight;
        }

        // If the top has been reached, it's time to return the current node weight, otherwise create the successor for this node
        if self.layer_number == TREE_HEIGHT {
            current_node_weight
        } else {
            let turn_for_bot = self.bot_color == turn;
            let mut result = if turn_for_bot { i16::MIN } else { i16::MAX };

            let legal_moves = chess.legal_moves();

            let mut moves_number = i16::default();

            if self.layer_number <= 2 {
                moves_number = legal_moves.len() as i16; // Will be needed to adjust the weight
            }

            for legal_move in &legal_moves {
                let pos_after_move = chess.clone().play(legal_move).unwrap();

                let mut node_weight = (Node {
                    fen: Epd::from_position(pos_after_move, EnPassantMode::Legal).to_string(),
                    layer_number: self.layer_number + 1,
                    bot_color: self.bot_color,
                    bot_wants_stalemate: self.bot_wants_stalemate,
                    previous_move: legal_move.clone(),
                    previous_weight: current_node_weight,
                })
                .get_node_weight();

                let node_weights_abs = node_weight.abs();

                if node_weights_abs != CHECKMATE_WEIGHT && node_weights_abs != STALEMATE_WEIGHT {
                    match self.layer_number {
                        1 => node_weight -= moves_number, // The more moves the opponent has, the worse
                        2 => node_weight += moves_number, // The more moves the bot has, the better
                        _ => (),
                    }
                }

                result = if turn_for_bot {
                    // Chosing the best move for the bot
                    result.max(node_weight)
                } else {
                    // Chosing the best move for the opponent
                    result.min(node_weight)
                }
            }

            result
        }
    }

    /// Get the weight of a specific piece.
    fn get_piece_weight(&self, piece: Role) -> i16 {
        match piece {
            Role::Pawn => PAWN_WEIGHT,
            Role::Knight => KNIGHT_WEIGHT,
            Role::Bishop => BISHOP_WEIGHT,
            Role::Queen => QUEEN_WEIGHT,
            Role::Rook => ROOK_WEIGHT,
            Role::King => 0, // Artificially avoiding "_ => ()"
        }
    }
}

/// Calculate the weight for either black or white using the given FEN.
fn get_weight_by_fen(fen: &str, bot_color: Color) -> i16 {
    let mut weight_for_white: i16 = 0;

    for piece in fen.split_once(' ').unwrap().0.chars() {
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

    if bot_color == Color::White {
        weight_for_white
    } else {
        -weight_for_white
    }
}
#[tauri::command(async, rename_all = "snake_case")]
/// Get the best move for the given FEN.
async fn get_move(current_fen: String) -> String {
    // Create an instance of Chess with the current FEN
    let fen: Fen = current_fen.parse().unwrap();
    let chess: Chess = fen.clone().into_position(CastlingMode::Standard).unwrap();

    let bot_color = chess.turn(); // The current turn/color the bot has to work with
    let weight_by_fen = get_weight_by_fen(&current_fen, bot_color);

    let legal_moves = chess.legal_moves();

    let mut move_weights = HashMap::with_capacity(legal_moves.len()); // { move: weight of the move }

    // Start to fill the hashmap (working with the first layer)
    for legal_move in legal_moves {
        let pos_after_move = chess.clone().play(&legal_move).unwrap(); // Get the position (Chess
                                                                       // insance after a hypothetical move)

        // Start with the root moves that continue the tree and eventually return the weight
        move_weights.insert(
            legal_move.clone(),
            (Node {
                fen: Epd::from_position(pos_after_move, EnPassantMode::Legal).to_string(),
                layer_number: 1,
                bot_color,
                bot_wants_stalemate: weight_by_fen <= -ROOK_WEIGHT, // It's been experimentally determined that when the bot doesn't have a rook, it should want to get a stalemate
                previous_move: legal_move.clone(),
                previous_weight: weight_by_fen,
            })
            .get_node_weight(),
        );
    }

    // Retain the moves with the best final weight
    let max_weight_move = move_weights.iter().max_by_key(|entry| entry.1).unwrap().1;

    // Keep the elements with the best weight in the hashmap
    let mut filtered_move_weights: HashMap<&Move, &i16> = HashMap::new();
    for move_weight in &move_weights {
        if move_weight.1 == max_weight_move {
            filtered_move_weights.insert(move_weight.0, move_weight.1);
        }
    }

    // Get a random move from the collection of the best moves to make the way the bot plays
    // arbitrary
    return filtered_move_weights
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
