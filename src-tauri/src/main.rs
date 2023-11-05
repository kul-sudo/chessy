// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::{SystemTime, UNIX_EPOCH};
use serde::Serialize;
use shakmaty::{Chess, fen::{Fen, Epd}, CastlingMode, Position};

static TREE_HEIGHT: u16 = 6;

#[derive(Serialize)]
struct CurrentPosition {
    fen: String,
    layer_number: u16,
}

impl CurrentPosition {
    fn create_next_positions(&self) {
        // println!("{:?}", self.layer_number);
        let fen: Fen = self.fen.parse().unwrap();
        let position: Chess = fen.clone().into_position(CastlingMode::Standard).unwrap();
        
        if position.is_checkmate() {
            return
        }

        if position.is_stalemate() {
            return
        }

        if self.layer_number < TREE_HEIGHT {
            let legal_moves = position.legal_moves();
           
            for legal_move in legal_moves {
                let pos_after_move = position.clone().play(&legal_move).unwrap();
                let new_position = CurrentPosition {
                    fen: Epd::from_position(pos_after_move, shakmaty::EnPassantMode::Legal).to_string(),
                    layer_number: self.layer_number + 1
                };

                if self.layer_number < TREE_HEIGHT - 1 {
                    new_position.create_next_positions()
                }
            }
        }
    }
}

#[tauri::command(async)]
async fn create_tree() {
    let position = CurrentPosition {
        fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1".to_string(),
        layer_number: 0
    };

    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    position.create_next_positions();
    println!("time {:?}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - time)
}

#[tokio::main]
async fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        create_tree
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
