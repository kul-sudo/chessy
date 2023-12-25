static NANOSECOND: u128 = 1000000000;

static MAX_TIME_SECS: u128 = 3; // The time limit for handling one move of the bot in seconds
pub static TIME_TO_THINK: u128 = MAX_TIME_SECS * NANOSECOND; // In nanoseconds

/// The limit for the height of the tree.
pub static MAX_TREE_HEIGHT: i16 = 9;

pub static MIN_TREE_HEIGHT: i16 = 4; // >= 3

/// How many legal moves there can approximately be.
pub static MAX_LEGAL_MOVES: i16 = 100;

/// How many moves an opening is supposed to take.
pub static MAX_OPENING_MOVES: u32 = 20;

// Piece weights
pub static PAWN_WEIGHT: i16 = 3 * MAX_LEGAL_MOVES + 1; // 1 * (3 * MAX_LEGAL_MOVES + 1)
pub static KNIGHT_WEIGHT: i16 = 3 * PAWN_WEIGHT;
pub static BISHOP_WEIGHT: i16 = 3 * PAWN_WEIGHT;
pub static QUEEN_WEIGHT: i16 = 9 * PAWN_WEIGHT;
pub static ROOK_WEIGHT: i16 = 5 * PAWN_WEIGHT;

pub static INFINITY: i16 = i16::MAX;
pub static CHECKMATE_WEIGHT: i16 = INFINITY - 1;
pub static STALEMATE_WEIGHT: i16 = CHECKMATE_WEIGHT - 1;

