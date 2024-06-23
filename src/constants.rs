use once_cell::sync::Lazy;
use shakmaty::fen::Fen;

pub const CHESS_FIELD_ROWS: usize = 8;
pub const CHESS_FIELD_COLUMNS: usize = CHESS_FIELD_ROWS;

const NANOSECOND: u128 = 1000000000;

const MAX_TIME_SECS: u128 = 1; // The time limit for handling one move of the bot in seconds
pub static TIME_TO_THINK: u128 = MAX_TIME_SECS * NANOSECOND; // In nanoseconds

pub const MIN_TREE_HEIGHT: i32 = 4; // >= 3

/// How many moves an opening is supposed to take.
pub const MAX_OPENING_MOVES: u32 = 20;

pub const INFINITY: i32 = i32::MAX;
pub static CHECKMATE_WEIGHT_STARTING_POINT: i32 = INFINITY - 1;

pub static DRAW_LEVEL: i32 = -ROOK_WEIGHT;

// The number of legal moves a piece can make on an empty board
const MAX_KNIGHT_MOVES: i32 = 8;
static MAX_BISHOP_MOVES: i32 = 7 * 2;
static MAX_ROOK_MOVES: i32 = 7 * 2;
static MAX_QUEEN_MOVES: i32 = 7 * 4;
const MAX_KING_MOVES: i32 = 8;

/// How many legal moves there can be. Equal to 332.
pub static MAX_LEGAL_MOVES: i32 = MAX_KNIGHT_MOVES * 2
    + MAX_BISHOP_MOVES * 2
    + MAX_ROOK_MOVES * 2
    + MAX_KING_MOVES
    + MAX_QUEEN_MOVES * 9;

// Piece weights
pub static PAWN_WEIGHT: i32 = 3 * (MAX_LEGAL_MOVES + 1); // This value ensures that the different weights of the two final positions remain different after adjusting the number of moves.
pub static KNIGHT_WEIGHT: i32 = 3 * PAWN_WEIGHT;
pub static BISHOP_WEIGHT: i32 = 3 * PAWN_WEIGHT;
pub static QUEEN_WEIGHT: i32 = 9 * PAWN_WEIGHT;
pub static ROOK_WEIGHT: i32 = 5 * PAWN_WEIGHT;

// IT'S IMPORTANT FOR THE VARIABLES TO BE i32, BECAUSE THE WEIGHT CAN HAVE THE MAXIMUM VALUE OF 102897.
