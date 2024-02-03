#[macro_export]
macro_rules! handle_checkmate_or_draw {
    ($chess:expr, $bot_turn:expr, $layer_number:expr) => {
        // Handle a possible checkmate
        if $chess.is_checkmate() {
            let checkmate_weight_for_this_layer = CHECKMATE_WEIGHT_STARTING_POINT - $layer_number;
            // Return the worst or best weight depending on who the checkmate has been performed by
            return RatingOrMove::Rating(if $bot_turn {
                -checkmate_weight_for_this_layer
            } else {
                checkmate_weight_for_this_layer
            });
        }

        // Handle a possible draw
        if $chess.halfmoves() == 50 || $chess.is_stalemate() {
            let draw_weight_for_this_layer = unsafe { DRAW_WEIGHT_STARTING_POINT } - $layer_number;
            // Return the worst or best weight depending on whether the bot wants a draw;
            // however, a checkmate has a higher weight than a draw
            return RatingOrMove::Rating(if unsafe { BOT_WANTS_DRAW } {
                draw_weight_for_this_layer
            } else {
                -draw_weight_for_this_layer
            });
        }
    };
}

#[macro_export]
/// If there's no checkmate or stalemate, the rating is corrected according
/// to the number of moves of the bot and the opponent.
macro_rules! correct_rating {
    ($rating_to_return:expr, $fen:expr, $bot_color:expr, $opening_is_going:expr, $moves_number:expr, $layer_number:expr) => {
        if $rating_to_return.abs() < unsafe { DRAW_WEIGHT_STARTING_POINT - TREE_HEIGHT } {
            // ^ Making sure $child_node_rating is neither a checkmate nor a staltemate
            $rating_to_return += match $layer_number {
                1 => {
                    -2 * (if {
                        let position =
                            get_only_position!($fen.to_string())
                            .to_string();

                        (match $bot_color {
                            Color::White => POSITIONS_IN_CHECK_B.lock(),
                            Color::Black => POSITIONS_IN_CHECK_W.lock()
                        }).unwrap().contains(&position)
                    } {
                        MAX_LEGAL_MOVES // Suppressing repeating checks
                    } else {
                        $moves_number
                    })
                }, // It's been experimentally detemined that the more moves the opponent has, the worse.
                2 if $opening_is_going && unsafe { !FIRST_QUEEN_OR_KING_MOVE } => {
                    // Needed during the opening for the bot to develop its pieces;
                    // however, after the end of the opening, it may cause endless repetitive moves
                    $moves_number
                } // The more moves the bot has, the better
                _ => 0,
            }
        } // Finish the correction
    };
}

#[macro_export]
macro_rules! optimise {
    ($current_rating:expr, $child_node_rating:expr, $bot_turn:expr, $previous_current_rating:expr, $layer_number:expr) => {
        // Start the optimisation
        $current_rating = (if $bot_turn { max } else { min })($current_rating, $child_node_rating);

        match $layer_number {
            1 => {
                if $current_rating < $previous_current_rating {
                    return RatingOrMove::Rating(-INFINITY);
                }
            }
            2.. => {
                if $bot_turn {
                    if $current_rating >= $previous_current_rating {
                        return RatingOrMove::Rating(INFINITY);
                    }
                } else if $current_rating <= $previous_current_rating {
                    return RatingOrMove::Rating(-INFINITY);
                }
            }
            _ => (),
        }
        // End the optimisation
    };
}

#[macro_export]
/// Get the weight of a specific piece.
macro_rules! get_piece_weight {
    ($piece:expr) => {
        match $piece {
            Role::Pawn => PAWN_WEIGHT,
            Role::Knight => KNIGHT_WEIGHT,
            Role::Bishop => BISHOP_WEIGHT,
            Role::Queen => QUEEN_WEIGHT,
            Role::Rook => ROOK_WEIGHT,
            _ => 0,
        }
    };
}

#[macro_export]
macro_rules! queen_or_king_first_move_handle {
    ($layer_is_0:expr, $legal_move:expr) => {
        if $layer_is_0 {
            unsafe {
                FIRST_QUEEN_OR_KING_MOVE = false;
            }

            match $legal_move.role() {
                Role::King | Role::Queen => unsafe { FIRST_QUEEN_OR_KING_MOVE = true },
                _ => (),
            }
        }
    };
}
