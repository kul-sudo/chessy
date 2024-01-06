// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{cmp::Ordering, num::NonZeroU32, time::Instant};

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
    let one_node_handle_time_estimated = unsafe { ONE_NODE_HANDLE_TIME } != -1.0;
    let tree_height;

    match one_node_handle_time_estimated {
        true => {
            let branching_rate = unsafe { BRANCHING_RATE };
            if branching_rate > 1.0 {
                let mut height_estimation =
                    ((TIME_TO_THINK as f64) / unsafe { ONE_NODE_HANDLE_TIME }).log(branching_rate);

                height_estimation = height_estimation.max(MIN_TREE_HEIGHT as f64); // If the
                                                                                   // estimation value is too low
                let current_tree_height = unsafe { TREE_HEIGHT };

                tree_height = match (height_estimation as i16).cmp(&current_tree_height) {
                    Ordering::Greater => current_tree_height + 1,
                    Ordering::Less => current_tree_height - 1,
                    Ordering::Equal => current_tree_height,
                }
            } else {
                tree_height = MIN_TREE_HEIGHT
            }
        }
        false => {
            tree_height = MIN_TREE_HEIGHT;
            unsafe { NODES_NUMBER = 0 }
        }
    };
    // Finished defining the height of the tree
    unsafe { STALEMATE_WEIGHT = CHECKMATE_WEIGHT - 1 - TREE_HEIGHT }

    let _ = app_handle.emit_all("log", tree_height.to_string());

    unsafe {
        TREE_HEIGHT = tree_height;
        BOT_COLOR = bot_color;
        BOT_WANTS_STALEMATE = weight_by_fen <= -ROOK_WEIGHT;
        OPENING_IS_GOING = fullmoves <= NonZeroU32::new(MAX_OPENING_MOVES).unwrap();
    }

    let now = Instant::now();
    let move_to_return;

    if let RatingOrMove::Move(value) = (Node {
        fen,
        layer_number: 0,
        weight: weight_by_fen,
        previous_current_rating: INFINITY,
    })
    .get_node_rating_or_move()
    {
        tree_building_time = now.elapsed().as_nanos();
        move_to_return = value.to_string();

        // If one node handle time hasn't been estimated, we have to do it
        if !one_node_handle_time_estimated {
            let one_node_handle_time_value =
                (tree_building_time as f64) / (unsafe { NODES_NUMBER } as f64);
            unsafe { ONE_NODE_HANDLE_TIME = one_node_handle_time_value }
        }

        let branching_rate_value = ((tree_building_time as f64) / unsafe { ONE_NODE_HANDLE_TIME })
            .powf(1.0 / (tree_height as f64));
        unsafe { BRANCHING_RATE = branching_rate_value }
    } else {
        unreachable!()
    }

    move_to_return
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_move])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
