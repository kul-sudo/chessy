// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::num::NonZeroU32;

mod constants;
mod mut_static;
mod node;
mod utils;

use constants::*;
use mut_static::*;
use utils::*;

use node::Node;
use shakmaty::{fen::Fen, CastlingMode, Chess, Position};

#[tauri::command(async, rename_all = "snake_case")]
/// Get the best move for the given FEN.
async fn get_move(current_fen: String) -> String {
    // Create an instance of Chess with the current FEN
    let fen: Fen = current_fen.parse().unwrap();
    let chess: Chess = fen.clone().into_position(CastlingMode::Standard).unwrap();

    let bot_color = chess.turn(); // The current turn/color the bot has to work with

    let weight_by_fen = get_weight_by_fen(fen.clone(), bot_color);
    let fullmoves = chess.fullmoves();

    unsafe {
        BOT_COLOR = bot_color;
        BOT_WANTS_STALEMATE = weight_by_fen <= -ROOK_WEIGHT;
        OPENING_IS_GOING = fullmoves <= NonZeroU32::new(MAX_OPENING_MOVES).unwrap()
    }

    if let RatingOrMove::Move(value) = (Node {
        fen,
        layer_number: 0,
        previous_move: None,
        previous_weight: weight_by_fen, // !!! In the root, previous_weight is equal to the weight of the root
        previous_current_rating: -INFINITY,
    })
    .get_node_rating_or_move()
    {
        value.to_string()
    } else {
        unreachable!()
    }
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_move])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
