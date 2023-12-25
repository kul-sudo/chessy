use shakmaty::Color;

pub static mut FIRST_MOVE: bool = true;

pub static mut TREE_HEIGHT: i16 = 4;

pub static mut ONE_NODE_HANDLE_TIME: f64 = 0.0;

pub static mut BRANCHING_RATE: f64 = 0.0;

pub static mut NODES_NUMBER: u16 = 0;

/// The color the bot is playing on.
pub static mut BOT_COLOR: Color = Color::White;

/// It's been experimentally determined that when the bot doesn't have a rook, it should want to get a stalemate.
pub static mut BOT_WANTS_STALEMATE: bool = false;

/// Whether the opening part of the game is still going.
pub static mut OPENING_IS_GOING: bool = false;

