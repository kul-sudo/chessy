// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashMap, num::NonZeroU32};

use rand::seq::IteratorRandom;
use shakmaty::{
    fen::{Epd, Fen},
    CastlingMode, Chess, Color, EnPassantMode, Move, Position, Role,
};

static TREE_HEIGHT: i16 = 4; // It has to be either equal to or greater than 3

static MAX_LEGAL_MOVES: i16 = 100;

/// How many moves an opening is supposed to take.
static MAX_OPENING_MOVES: u32 = 20;

// Piece weights
static PAWN_WEIGHT: i16 = 3 * MAX_LEGAL_MOVES + 1; // 1 * (3 * MAX_LEGAL_MOVES + 1)
static KNIGHT_WEIGHT: i16 = 3 * PAWN_WEIGHT;
static BISHOP_WEIGHT: i16 = 3 * PAWN_WEIGHT;
static QUEEN_WEIGHT: i16 = 9 * PAWN_WEIGHT;
static ROOK_WEIGHT: i16 = 5 * PAWN_WEIGHT;

static CHECKMATE_WEIGHT: i16 = i16::MAX;
static STALEMATE_WEIGHT: i16 = CHECKMATE_WEIGHT - 1;

/// The color the bot is playing on.
static mut BOT_COLOR: Color = Color::White;

/// It's been experimentally determined that when the bot doesn't have a rook, it should want to get a stalemate.
static mut BOT_WANTS_STALEMATE: bool = false;

/// Whether the opening part of the game is still going.
static mut OPENING_IS_GOING: bool = false;

/// Node struct.
struct Node {
    /// The FEN of the node.
    fen: String,

    /// The layer number the node is currently at.
    layer_number: i16,

    /// The move of the node edge (every egde connects 2 nodes).
    previous_move: Move,

    /// The weight of the predecessor node.
    previous_weight: i16,
}

impl Node {
    /// Get the rating of the current node (the final weight when both the bot and the opponent play in the best way possible);
    /// this weight may be adjusted according to the number of the legal moves that can me made by either the bot or the opponent.
    fn get_node_rating(self) -> i16 {
        // Create an instance of Chess with the current FEN
        let fen: Fen = self.fen.parse().unwrap();
        let chess: Chess = fen.clone().into_position(CastlingMode::Standard).unwrap();

        let bot_color = unsafe { BOT_COLOR };
        let bot_wants_stalemate = unsafe { BOT_WANTS_STALEMATE };
        let opening_is_going = unsafe { OPENING_IS_GOING };

        let turn = chess.turn();

        // Handle a possible checkmate
        if chess.is_checkmate() {
            // Return the worst or best weight depending on who the checkmate has been performed by
            return if bot_color == turn {
                -CHECKMATE_WEIGHT
            } else {
                CHECKMATE_WEIGHT
            };
        }

        // Handle a possible stalemate
        if chess.is_stalemate() {
            // Return the worst or best weight depending on whether the bot wants a stalemate;
            // however, a checkmate has a higher weight than a stalemate
            return if bot_wants_stalemate {
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

            let coefficient = if turn == bot_color { -1 } else { 1 }; // Defines whether a capture or promotion is good for the bot depending on the turn/color

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
            let turn_for_bot = bot_color == turn;
            let mut result = if turn_for_bot { i16::MIN } else { i16::MAX };

            let legal_moves = chess.legal_moves();

            let moves_number = if self.layer_number <= 2 {
                legal_moves.len() as i16
            } else {
                i16::default()
            };

            for legal_move in &legal_moves {
                let pos_after_move = chess.clone().play(legal_move).unwrap();

                let mut node_rating = (Node {
                    fen: Epd::from_position(pos_after_move, EnPassantMode::Legal).to_string(),
                    layer_number: self.layer_number + 1,
                    previous_move: legal_move.clone(),
                    previous_weight: current_node_weight,
                })
                .get_node_rating();

                let node_rating_abs = node_rating.abs();

                if node_rating_abs != CHECKMATE_WEIGHT && node_rating_abs != STALEMATE_WEIGHT {
                    node_rating += match self.layer_number {
                        1 => -(2 * moves_number), // The more moves the opponent has, the worse
                        2 => {
                            // Needed during the opening for the bot to develop its pieces;
                            // however, after the end of the opening, it causes endless repetitive moves
                            if opening_is_going {
                                moves_number
                            } else {
                                0
                            }
                        } // The more moves the bot has, the better
                        _ => 0,
                    };
                }

                result = if turn_for_bot {
                    // Chosing the best move for the bot
                    result.max(node_rating)
                } else {
                    // Chosing the best move for the opponent
                    result.min(node_rating)
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
            Role::King => 0,
        }
    }
}

/// Calculate the weight for either black or white using the given FEN.
fn get_weight_by_fen(fen: &str, bot_color: Color) -> i16 {
    let mut weight_for_white = i16::default();

    for piece in fen.split_once(' ').unwrap().0.chars() {
        weight_for_white += match piece {
            'P' => PAWN_WEIGHT,
            'R' => ROOK_WEIGHT,
            'Q' => QUEEN_WEIGHT,
            'B' => BISHOP_WEIGHT,
            'N' => KNIGHT_WEIGHT,
            'p' => -PAWN_WEIGHT,
            'r' => -ROOK_WEIGHT,
            'q' => -QUEEN_WEIGHT,
            'b' => -BISHOP_WEIGHT,
            'n' => -KNIGHT_WEIGHT,
            _ => 0,
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
    let fullmoves = chess.fullmoves();

    unsafe { BOT_COLOR = bot_color }
    unsafe { BOT_WANTS_STALEMATE = weight_by_fen <= -ROOK_WEIGHT }
    unsafe { OPENING_IS_GOING = fullmoves <= NonZeroU32::new(MAX_OPENING_MOVES).unwrap() }

    let legal_moves = chess.legal_moves();

    let mut move_ratings = HashMap::with_capacity(legal_moves.len()); // { move: rating of the move }

    // Start to fill the hashmap (working with the first layer)
    for legal_move in legal_moves {
        let pos_after_move = chess.clone().play(&legal_move).unwrap(); // Get the position (Chess
                                                                       // insance after a hypothetical move)

        // Start with the root moves that continue the tree and eventually return the rating
        move_ratings.insert(
            legal_move.clone(),
            (Node {
                fen: Epd::from_position(pos_after_move, EnPassantMode::Legal).to_string(),
                layer_number: 1,
                previous_move: legal_move.clone(),
                previous_weight: weight_by_fen,
            })
            .get_node_rating(),
        );
    }

    // Retain the moves with the best final rating
    let max_rating_move = move_ratings.iter().max_by_key(|entry| entry.1).unwrap().1;

    // Keep the elements with the best rating
    let mut filtered_move_ratings: HashMap<&Move, &i16> = HashMap::new();

    for move_rating in &move_ratings {
        if move_rating.1 == max_rating_move {
            filtered_move_ratings.insert(move_rating.0, move_rating.1);
        }
    }

    // Get a random move from the collection of the best moves to make the way the bot plays arbitrary
    return filtered_move_ratings
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
