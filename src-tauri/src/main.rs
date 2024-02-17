// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{cmp::Ordering, num::NonZeroU32, time::Instant};

mod constants;
mod mut_static;
mod node;
mod node_macros;
mod opening_book;
mod utils;

use constants::*;
use mut_static::*;
use tauri::{AppHandle, Manager};
use utils::*;

use node::Node;
use shakmaty::{fen::Fen, CastlingMode, Chess, Color, EnPassantMode, Position};

fn add_to_previous_positions(new_fen: Fen) {
    PREVIOUS_POSITIONS
        .lock()
        .unwrap()
        .entry(get_extended_position!(new_fen.to_string()))
        .and_modify(|value| *value += 1)
        .or_insert(1);

    let _ = LAST_PROCESSED_FEN.set(Some(new_fen));
}

macro_rules! fen_repeated_5_times {
    ($fen:expr) => {{
        if *PREVIOUS_POSITIONS
            .lock()
            .unwrap()
            .get(&get_extended_position!($fen.to_string()))
            .unwrap()
            == 5
        {
            return "5-repetition".to_string();
        }
    }};
}

#[tauri::command(async, rename_all = "snake_case")]
/// Get the best move for the given FEN.
async fn get_move(app_handle: AppHandle, current_fen: String) -> String {
    // Create an instance of Chess with the current FEN
    let fen = current_fen.parse::<Fen>().unwrap();

    let mut chess: Chess = fen.clone().into_position(CastlingMode::Standard).unwrap();

    let tree_building_time: u128;

    let bot_color = chess.turn(); // The current turn/color the bot has to work with

    let root_weight = get_weight_by_fen(current_fen, bot_color);
    let fullmoves = chess.fullmoves();

    // Reset the value
    unsafe { NODES_NUMBER = 0 }

    // Start defining the height of the tree
    let tree_height;

    match fullmoves == NonZeroU32::new(1).unwrap() {
        true => {
            tree_height = MIN_TREE_HEIGHT;

            unsafe {
                clear_positions_in_check!();
                match bot_color {
                    Color::White => PREVIOUS_TREE_HEIGHT_WHITE = MIN_TREE_HEIGHT,
                    Color::Black => PREVIOUS_TREE_HEIGHT_BLACK = MIN_TREE_HEIGHT,
                };
            }

            if chess.halfmoves() == 0 {
                unsafe { FIRST_MOVE_MADE_BY_BOT = true }
            }

            // The bot's playing as black against a real person playing as white
            let condition = bot_color == Color::Black && unsafe { !FIRST_MOVE_MADE_BY_BOT };

            if bot_color == Color::White || condition {
                // Clean up
                PREVIOUS_POSITIONS.lock().unwrap().clear();
                let _ = LAST_PROCESSED_FEN.set(None);

                if condition {
                    add_to_previous_positions(
                        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
                            .parse::<Fen>()
                            .unwrap(),
                    )
                }
            }
        }
        false => {
            // Prepare for the next game
            unsafe { FIRST_MOVE_MADE_BY_BOT = false }

            if unsafe {
                match BOT_COLOR {
                    Color::Black => LAST_MOVE_FROM_BOOK_B,
                    Color::White => LAST_MOVE_FROM_BOOK_W,
                }
            } {
                tree_height = MIN_TREE_HEIGHT
            } else {
                let branching_rate = unsafe {
                    match bot_color {
                        Color::White => BRANCHING_RATE_WHITE,
                        Color::Black => BRANCHING_RATE_BLACK,
                    }
                };

                if branching_rate > 1.0 {
                    let mut height_estimation = ((TIME_TO_THINK as f64)
                        / unsafe { ONE_NODE_HANDLE_TIME })
                    .log(branching_rate);

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
        }
    }

    if let Some(last_processed_fen) = LAST_PROCESSED_FEN.get() {
        // 1. The bot's playing as white, and it's now its first move.
        // OR
        // 2. It is not the first move of the game for the bot, and the previous move was done
        //    by a real person
        if last_processed_fen.clone().is_some_and(|x| fen != x) {
            add_to_previous_positions(fen.clone());
            fen_repeated_5_times!(fen);
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
        BOT_WANTS_DRAW = root_weight <= DRAW_LEVEL || chess.has_insufficient_material(bot_color);
        OPENING_IS_GOING = fullmoves <= NonZeroU32::new(MAX_OPENING_MOVES).unwrap();
        DRAW_WEIGHT_STARTING_POINT = CHECKMATE_WEIGHT_STARTING_POINT - (TREE_HEIGHT + 1);
        (LAST_MOVE_FROM_BOOK_W, LAST_MOVE_FROM_BOOK_B) = (true, true);
    }

    let now = Instant::now();
    let move_to_return;

    if chess.halfmoves() == 0 {
        clear_positions_in_check!();
    }

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

    if move_to_return.is_zeroing() {
        clear_positions_in_check!();
    }

    let chess_after_move = {
        chess.play_unchecked(&move_to_return);
        chess
    };

    let fen_after_move = Fen::from_position(chess_after_move.clone(), EnPassantMode::Legal);

    if unsafe { !BOT_WANTS_DRAW } && chess_after_move.is_check() {
        let position = get_only_position!(fen_after_move.to_string()).to_string();

        (match bot_color {
            Color::White => POSITIONS_IN_CHECK_B.lock(),
            Color::Black => POSITIONS_IN_CHECK_W.lock(),
        })
        .unwrap()
        .push(position);
    }

    add_to_previous_positions(fen_after_move.clone());
    fen_repeated_5_times!(fen_after_move);

    move_to_return.to_string()
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_move])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
