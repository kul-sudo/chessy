// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{num::NonZeroU32, time::Instant};

mod constants;
mod mut_static;
mod node;
mod utils;

use constants::*;
use mut_static::*;
use tauri::{AppHandle, Manager};
use utils::*;

use node::Node;
use shakmaty::{fen::Fen, CastlingMode, Chess, Position};

#[tauri::command(async)]
async fn first_move() {
    unsafe { FIRST_MOVE = true }
}

#[tauri::command(async, rename_all = "snake_case")]
/// Get the best move for the given FEN.
async fn get_move(app_handle: AppHandle, current_fen: String) -> String {
    // Create an instance of Chess with the current FEN
    let fen: Fen = current_fen.parse().unwrap();
    let chess: Chess = fen.clone().into_position(CastlingMode::Standard).unwrap();

    let tree_building_time: u128;

    let bot_color = chess.turn(); // The current turn/color the bot has to work with

    let weight_by_fen = get_weight_by_fen(fen.clone(), bot_color);
    let fullmoves = chess.fullmoves();

    // Start defining the height of the tree
    let first_move = unsafe { FIRST_MOVE };
    let mut tree_height = unsafe { TREE_HEIGHT };

    match first_move {
        true => tree_height = MIN_TREE_HEIGHT,
        false => {
            let time_delta: i128 =
                unsafe { (TIME_TO_THINK as i128) - (LAST_TREE_BUILDING_TIME as i128) };
            if time_delta > (TIME_COMPARISON_PRECISION as i128) {
                if tree_height < MAX_TREE_HEIGHT {
                    tree_height += 1
                }
            } else if time_delta < -(TIME_COMPARISON_PRECISION as i128) {
                tree_height -= 1
            }
        }
    };

    let _ = app_handle.emit_all("log", tree_height.to_string());

    unsafe { TREE_HEIGHT = tree_height }
    // Finished defining the height of the tree

    unsafe {
        BOT_COLOR = bot_color;
        BOT_WANTS_STALEMATE = weight_by_fen <= -ROOK_WEIGHT;
        OPENING_IS_GOING = fullmoves <= NonZeroU32::new(MAX_OPENING_MOVES).unwrap();
        if FIRST_MOVE {
            FIRST_MOVE = false
        }
    }

    let now = Instant::now();
    let move_to_return;

    if let RatingOrMove::Move(value) = (Node {
        fen,
        layer_number: 0,
        previous_move: None,
        previous_weight: weight_by_fen, // !!! In the root, previous_weight is equal to the weight of the root
        previous_current_rating: INFINITY,
    })
    .get_node_rating_or_move()
    {
        tree_building_time = now.elapsed().as_nanos();
        move_to_return = value.to_string()
    } else {
        unreachable!()
    }

    unsafe { LAST_TREE_BUILDING_TIME = tree_building_time }

    move_to_return
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_move, first_move])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
