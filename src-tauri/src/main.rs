// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{cmp::Ordering, num::NonZeroU32, time::Instant};

mod constants;
mod mut_static;
mod node;
mod node_macros;
mod utils;

use constants::*;
use mut_static::*;
use tauri::{AppHandle, Manager};
use utils::*;

use node::Node;
use shakmaty::{fen::Fen, CastlingMode, Chess, Color, Position};

#[tauri::command(async, rename_all = "snake_case")]
/// Get the best move for the given FEN.
async fn get_move(app_handle: AppHandle, current_fen: String) -> String {
    // let mut previous_checks_w = PREVIOUS_CHECKS_W.lock().unwrap();
    // println!("{:?}", previous_checks_w);
    // previous_checks_w.insert(8_i16, 8_i16);

    // Create an instance of Chess with the current FEN
    let fen = current_fen.parse::<Fen>().unwrap();
    let chess: Chess = fen.clone().into_position(CastlingMode::Standard).unwrap();

    let tree_building_time: u128;

    let bot_color = chess.turn(); // The current turn/color the bot has to work with

    let mut previous_checks = (match bot_color {
        Color::White => PREVIOUS_CHECKS_W.lock(),
        Color::Black => PREVIOUS_CHECKS_B.lock(),
    })
    .unwrap();

    let root_weight = get_weight_by_fen(fen.clone(), bot_color);
    let fullmoves = chess.fullmoves();

    // Start defining the height of the tree
    let tree_height;

    // Reset the value
    unsafe { NODES_NUMBER = 0 }

    if fullmoves == NonZeroU32::new(1).unwrap() {
        // Making sure the game has just begun prevents the issue when the brancing rate from the
        // previous game was used in the new one for the bot of this colour.
        tree_height = MIN_TREE_HEIGHT;
        unsafe {
            match bot_color {
                Color::White => PREVIOUS_TREE_HEIGHT_WHITE = MIN_TREE_HEIGHT,
                Color::Black => PREVIOUS_TREE_HEIGHT_BLACK = MIN_TREE_HEIGHT,
            }
        }
    } else {
        let branching_rate = unsafe {
            match bot_color {
                Color::White => BRANCHING_RATE_WHITE,
                Color::Black => BRANCHING_RATE_BLACK,
            }
        };

        if branching_rate > 1.0 {
            let mut height_estimation =
                ((TIME_TO_THINK as f64) / unsafe { ONE_NODE_HANDLE_TIME }).log(branching_rate);

            height_estimation = height_estimation.max(MIN_TREE_HEIGHT as f64); // If the
                                                                               // estimation value is too low
            let current_tree_height = unsafe { TREE_HEIGHT };

            tree_height = current_tree_height
                + match (height_estimation as i32).cmp(&current_tree_height) {
                    Ordering::Greater => 1,
                    Ordering::Less => -1,
                    _ => 0,
                }
        } else {
            tree_height = unsafe {
                match bot_color {
                    Color::White => PREVIOUS_TREE_HEIGHT_WHITE,
                    Color::Black => PREVIOUS_TREE_HEIGHT_BLACK,
                }
            }
        }
    }

    unsafe {
        match bot_color {
            Color::White => PREVIOUS_TREE_HEIGHT_WHITE = tree_height,
            Color::Black => PREVIOUS_TREE_HEIGHT_BLACK = tree_height,
        }
    }

    // Finished defining the height of the tree
    let _ = app_handle.emit_all("log", tree_height.to_string());

    unsafe {
        TREE_HEIGHT = tree_height;
        BOT_COLOR = bot_color;
        BOT_WANTS_STALEMATE = root_weight <= -ROOK_WEIGHT;
        OPENING_IS_GOING = fullmoves <= NonZeroU32::new(MAX_OPENING_MOVES).unwrap();
        STALEMATE_WEIGHT_STARTING_POINT = CHECKMATE_WEIGHT_STARTING_POINT - (1 + TREE_HEIGHT)
    }

    if chess.halfmoves() <= 1
    // If the bot or its opponent has captured a piece or moved a pawn.
    // The previous positions can't repeat.
    {
        previous_checks.clear()
    }

    let now = Instant::now();
    let move_to_return;

    if let RatingOrMove::Move(value) = (Node {
        fen,
        layer_number: 0,
        weight: root_weight,
        previous_current_rating: INFINITY,
    })
    .get_node_rating_or_move()
    {
        tree_building_time = now.elapsed().as_nanos();
        move_to_return = value;

        let tree_building_time_f64 = tree_building_time as f64;

        let one_node_handle_time_value =
            (tree_building_time_f64) / (unsafe { NODES_NUMBER } as f64);
        unsafe { ONE_NODE_HANDLE_TIME = one_node_handle_time_value }

        let branching_rate_value = ((tree_building_time_f64) / unsafe { ONE_NODE_HANDLE_TIME })
            .powf(1.0 / (tree_height as f64));
        unsafe {
            match bot_color {
                Color::White => BRANCHING_RATE_WHITE = branching_rate_value,
                Color::Black => BRANCHING_RATE_BLACK = branching_rate_value,
            }
        }
    } else {
        unreachable!()
    }

    // Changing the hashmap
    let position = position_from_fen!(current_fen);

    chess.play_unchecked(&move_to_return);
    if root_weight.is_positive() && chess.is_check() {
        // if previous_checks.contains_key(position) {
        //     let old_vec = previous_checks.get_mut(position).unwrap();
        //     old_vec.push(move_to_return);
        //     previous_checks.insert(position, old_vec);
        // }
        // previous_checks.(key, value)
        // previous_checks.insert('')
    }
    // previous_checks.insert(, v)
    move_to_return.to_string()
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_move])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
