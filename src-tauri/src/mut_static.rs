use shakmaty::Color;

pub static mut FIRST_MOVE: bool = true;

pub static mut TREE_HEIGHT: i16 = 4; // Has to be either equal to or greater than 3

pub static mut ONE_NODE_HANDLE_TIME: u128 = 0;

/// The color the bot is playing on.
pub static mut BOT_COLOR: Color = Color::White;

/// It's been experimentally determined that when the bot doesn't have a rook, it should want to get a stalemate.
pub static mut BOT_WANTS_STALEMATE: bool = false;

/// Whether the opening part of the game is still going.
pub static mut OPENING_IS_GOING: bool = false;
