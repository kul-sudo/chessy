use shakmaty::{Color, Move};

use crate::constants::*;

pub enum RatingOrMove {
    Rating(i32),
    Move(Move),
}

#[macro_export]
macro_rules! get_only_position {
    ($fen:expr) => {
        $fen.split_once(' ').unwrap().0
    };
}

#[macro_export]
macro_rules! clear_positions_in_check {
    () => {
        for positions_in_check in [&POSITIONS_IN_CHECK_W, &POSITIONS_IN_CHECK_B] {
            positions_in_check.lock().unwrap().clear()
        }
    };
}

/// Calculate the weight for either black or white using the given FEN.
pub fn get_weight_by_fen(fen: String, bot_color: Color) -> i32 {
    let weight_for_white = get_only_position!(fen)
        .chars()
        .map(|piece| match piece {
            'P' => PAWN_WEIGHT,
            'R' => ROOK_WEIGHT,
            'Q' => QUEEN_WEIGHT,
            'B' => BISHOP_WEIGHT,
            'N' => KNIGHT_WEIGHT,
            'p' => -PAWN_WEIGHT,
            'r' => -ROOK_WEIGHT,
            'q' => -QUEEN_WEIGHT,
            'b' => -BISHOP_WEIGHT,
            'n' => -KNIGHT_WEIGHT,
            _ => 0,
        })
        .sum();

    match bot_color {
        Color::White => weight_for_white,
        Color::Black => -weight_for_white,
    }
}
