// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashMap, num::NonZeroU32};

mod constants;
mod mut_static;
mod node;

use constants::*;
use mut_static::*;

use node::Node;
use rand::seq::IteratorRandom;
use shakmaty::{
    fen::{Epd, Fen},
    CastlingMode, Chess, Color, EnPassantMode, Move, Position,
};

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

    unsafe {
        BOT_COLOR = bot_color;
        BOT_WANTS_STALEMATE = weight_by_fen <= -ROOK_WEIGHT;
        OPENING_IS_GOING = fullmoves <= NonZeroU32::new(MAX_OPENING_MOVES).unwrap()
    }

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
            .get_node_rating()
            .parse::<i16>()
            .unwrap(),
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
    filtered_move_ratings
        .keys()
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string()
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_move])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
