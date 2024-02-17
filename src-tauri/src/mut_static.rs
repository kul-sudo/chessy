use once_cell::sync::{Lazy, OnceCell};
use shakmaty::{fen::Fen, Color};
use std::{collections::HashMap, sync::Mutex};

pub static mut LAST_MOVE_FROM_BOOK_W: bool = true;
pub static mut LAST_MOVE_FROM_BOOK_B: bool = true;

pub static mut TREE_HEIGHT: i32 = 4;

pub static POSITIONS_IN_CHECK_W: Lazy<Mutex<Vec<String>>> =
    Lazy::new(|| Mutex::new(Vec::with_capacity(25)));
pub static POSITIONS_IN_CHECK_B: Lazy<Mutex<Vec<String>>> =
    Lazy::new(|| Mutex::new(Vec::with_capacity(25)));
pub static PREVIOUS_POSITIONS: Lazy<Mutex<HashMap<String, u8>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
pub static LAST_PROCESSED_FEN: OnceCell<Option<Fen>> = OnceCell::new();

pub static mut FIRST_MOVE_MADE_BY_BOT: bool = false;

pub static mut ONE_NODE_HANDLE_TIME: f64 = 0.0;

pub static mut BRANCHING_RATE_WHITE: f64 = 0.0;
pub static mut BRANCHING_RATE_BLACK: f64 = 0.0;

pub static mut PREVIOUS_TREE_HEIGHT_WHITE: i32 = 0;
pub static mut PREVIOUS_TREE_HEIGHT_BLACK: i32 = 0;

pub static mut NODES_NUMBER: u32 = 0;

/// The color the bot is playing on.
pub static mut BOT_COLOR: Color = Color::White;

/// It's been experimentally determined that when the bot doesn't have a rook, it should want to get a stalemate.
pub static mut BOT_WANTS_DRAW: bool = false;

/// Whether the opening part of the game is still going.
pub static mut OPENING_IS_GOING: bool = false;

pub static mut DRAW_WEIGHT_STARTING_POINT: i32 = 0;

pub static mut FIRST_QUEEN_OR_KING_MOVE: bool = false;
