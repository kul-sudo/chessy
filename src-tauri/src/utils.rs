use shakmaty::{fen::Fen, Color, Move};

use crate::constants::*;

pub enum RatingOrMove {
    Rating(i32),
    Move(Move),
}

#[macro_export]
macro_rules! position_from_fen {
    ($fen:expr) => {
        $fen.to_string().split_once(' ').unwrap().0
    };
}

/// Calculate the weight for either black or white using the given FEN.
pub fn get_weight_by_fen(fen: Fen, bot_color: Color) -> i32 {
    let weight_for_white = position_from_fen!(fen)
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

    if bot_color == Color::White {
        weight_for_white
    } else {
        -weight_for_white
    }
}
