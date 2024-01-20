use lazy_static::lazy_static;
use shakmaty::{Color, Move};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

lazy_static! {
    // The capacity is equal to 25 due to the 50-move rule
    pub static ref PREVIOUS_CHECKS_W: Arc<Mutex<HashMap<String, Vec<Move>>>> =
        Arc::new(Mutex::new(HashMap::with_capacity(25)));
    pub static ref PREVIOUS_CHECKS_B: Arc<Mutex<HashMap<String, Vec<Move>>>> =
        PREVIOUS_CHECKS_W.clone();
}

pub static mut TREE_HEIGHT: i32 = 4;

pub static mut ONE_NODE_HANDLE_TIME: f64 = 0.0;

pub static mut BRANCHING_RATE_WHITE: f64 = 0.0;
pub static mut BRANCHING_RATE_BLACK: f64 = 0.0;

pub static mut PREVIOUS_TREE_HEIGHT_WHITE: i32 = 0;
pub static mut PREVIOUS_TREE_HEIGHT_BLACK: i32 = 0;

pub static mut NODES_NUMBER: u32 = 0;

/// The color the bot is playing on.
pub static mut BOT_COLOR: Color = Color::White;

/// It's been experimentally determined that when the bot doesn't have a rook, it should want to get a stalemate.
pub static mut BOT_WANTS_STALEMATE: bool = false;

/// Whether the opening part of the game is still going.
pub static mut OPENING_IS_GOING: bool = false;

pub static mut STALEMATE_WEIGHT_STARTING_POINT: i32 = 0;

pub static mut FIRST_QUEEN_OR_KING_MOVE: bool = false;
