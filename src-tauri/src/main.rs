// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{num::NonZeroU32, time::Instant};

mod constants;
mod mut_static;
mod node;
mod utils;

use constants::*;
use mut_static::*;
use utils::*;

use node::Node;
use shakmaty::{fen::Fen, CastlingMode, Chess, Position};

#[tauri::command(async)]
async fn first_move() {
    unsafe { FIRST_MOVE = true }
}

#[tauri::command(async, rename_all = "snake_case")]
/// Get the best move for the given FEN.
async fn get_move(current_fen: String) -> String {
    // Create an instance of Chess with the current FEN
    let fen: Fen = current_fen.parse().unwrap();
    let chess: Chess = fen.clone().into_position(CastlingMode::Standard).unwrap();

    let tree_building_time: u128;

    let bot_color = chess.turn(); // The current turn/color the bot has to work with

    let weight_by_fen = get_weight_by_fen(fen.clone(), bot_color);
    let fullmoves = chess.fullmoves();
    let mut one_node_handle_time: u128;

    // Start defining the height of the tree
    let first_move = unsafe { FIRST_MOVE };
    let first_moves_number = chess.legal_moves().len();
    let tree_height = match first_move {
        true => MIN_TREE_HEIGHT, // At the fist move, there's no assessment for ONE_NODE_HANDLE_TIME
        false => {
            if first_moves_number == 1 {
                MAX_TREE_HEIGHT // It's better not to assess the height of the tree
            } else {
                one_node_handle_time = unsafe { ONE_NODE_HANDLE_TIME };

                // Straightforward assessment of the tree height.
                let mut tree_height_assessment = ((((MAX_TIME / one_node_handle_time) as f64)
                    .log2())
                    / (((first_moves_number as f64) * CORRECTIONAL_COEFFICIENT).log2()))
                .floor() as i16;
                println!("tree_height_assessment = {:?}", tree_height_assessment);

                tree_height_assessment = tree_height_assessment.max(MIN_TREE_HEIGHT); // If the
                                                                                      // assessment turns out to be too low
                tree_height_assessment.min(MAX_TREE_HEIGHT) // If the assessment turns out to be
                                                            // too high
            }
        }
    };

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

    // Start assessing ONE_NODE_HANDLE_TIME
    one_node_handle_time = ((tree_building_time as f64)
        / ((first_moves_number as f64).powf(tree_height as f64)))
        as u128;
    unsafe { ONE_NODE_HANDLE_TIME = one_node_handle_time }
    // Finish assessing ONE_NODE_HANDLE_TIME

    println!("tree_height = {:?}", tree_height);
    println!("tree_building_time = {:?}", tree_building_time / 100000000);
    println!("----------------------------------------------------------");
    move_to_return
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_move, first_move])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
