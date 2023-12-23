use shakmaty::{fen::Fen, Color, Move};

use crate::constants::*;

pub enum RatingOrMove {
    Rating(i16),
    Move(Move),
}

/// Calculate the weight for either black or white using the given FEN.
pub fn get_weight_by_fen(fen: Fen, bot_color: Color) -> i16 {
    let mut weight_for_white = i16::default();

    for piece in fen.to_string().split_once(' ').unwrap().0.chars() {
        weight_for_white += match piece {
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
        }
    }

    if bot_color == Color::White {
        weight_for_white
    } else {
        -weight_for_white
    }
}

/// The function estimates the time it takes to handle one node.
/// * `h` - The height of the tree.
/// * `m` - The number of legal moves from the root.
/// * `t` - The time it takes to handle the whole tree.
pub fn one_node_estimate_handle_time(h: i16, m: i16, t: u128) -> u128 {
    (((t as f64).log2()) / ((h as f64) * (m as f64).log2())) as u128
}
