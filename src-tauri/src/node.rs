use std::collections::HashMap;

use rand::{seq::SliceRandom, thread_rng};
use shakmaty::{fen::Fen, CastlingMode, Chess, EnPassantMode, Move, Position, Role};

use crate::{constants::*, mut_static::*, utils::RatingOrMove};

/// Node struct.
pub struct Node {
    /// The FEN of the node.
    pub fen: Fen,

    /// The layer number the node is currently at.
    pub layer_number: i16,

    /// The move of the node edge (every egde connects 2 nodes).
    pub previous_move: Option<Move>,

    /// The weight of the predecessor node.
    pub previous_weight: i16,

    /// The current value of `b.current_rating` which is the parent of `a`; the value of
    /// `b.current_rating` at the moment when `a` was created.
    pub previous_current_rating: i16,
}

impl Node {
    /// Get the rating of the current node (the final weight when both the bot and the opponent play in the best way possible);
    /// this weight may be adjusted according to the number of the legal moves that can me made by either the bot or the opponent.
    pub fn get_node_rating_or_move(self) -> RatingOrMove {
        // Create an instance of Chess with the current FEN
        let chess: Chess = self
            .fen
            .clone()
            .into_position(CastlingMode::Standard)
            .unwrap();

        let bot_color = unsafe { BOT_COLOR };
        let bot_wants_stalemate = unsafe { BOT_WANTS_STALEMATE };

        let bot_turn = chess.turn() == bot_color;

        if self.layer_number > 0 {
            // JS isn't supposed to call Rust if there's a checkmate or stalemate
            // Handle a possible checkmate
            if chess.is_checkmate() {
                // Return the worst or best weight depending on who the checkmate has been performed by
                return RatingOrMove::Rating(if bot_turn {
                    -CHECKMATE_WEIGHT
                } else {
                    CHECKMATE_WEIGHT
                });
            }

            // Handle a possible stalemate
            if chess.is_stalemate() {
                // Return the worst or best weight depending on whether the bot wants a stalemate;
                // however, a checkmate has a higher weight than a stalemate
                return RatingOrMove::Rating(if bot_wants_stalemate {
                    STALEMATE_WEIGHT
                } else {
                    -STALEMATE_WEIGHT
                });
            }
        }

        // Handling all the other cases (everything all the way down)
        let mut current_rating = if bot_turn { -INFINITY } else { INFINITY }; // Needed for
                                                                              // optimisation

        let legal_moves = chess.legal_moves();
        let moves_number = legal_moves.len() as i16;
        let mut move_ratings = HashMap::with_capacity(moves_number as usize); // { move: rating of the move };
                                                                              // needed only for the root (layer number = 0)

        // Start handling the weight of the current node
        let mut current_node_weight = self.previous_weight;

        if self.layer_number > 0 {
            let previous_move = self.previous_move.as_ref().unwrap();
            let is_capture = previous_move.is_capture();
            let is_promotion = previous_move.is_promotion();

            if is_capture || is_promotion {
                // If there has either been a capture or promotion, an
                // adjustment is done

                let coefficient = if bot_turn { -1 } else { 1 }; // Defines whether a capture or promotion is good for the bot depending on the turn/color

                if is_capture {
                    // Adjust the weight as a result of the captured piece
                    current_node_weight +=
                        self.get_piece_weight(previous_move.capture().unwrap()) * coefficient
                }

                if is_promotion {
                    // Adjust the weight as a result of the promoted pawn
                    current_node_weight +=
                        (self.get_piece_weight(previous_move.promotion().unwrap()) - PAWN_WEIGHT)
                            * coefficient;
                }
            }
        } // Finish handling the weight

        if self.layer_number == unsafe { TREE_HEIGHT } {
            // If the top has been reached, it's time to return the current node weight,
            // otherwise create the successor for this node
            RatingOrMove::Rating(current_node_weight)
        } else {
            let opening_is_going = unsafe { OPENING_IS_GOING };

            let mut rating_to_return = if bot_turn { -INFINITY } else { INFINITY };

            let mut legal_moves_to_shuffle = legal_moves.clone();
            legal_moves_to_shuffle.shuffle(&mut thread_rng());

            for legal_move in legal_moves_to_shuffle {
                let pos_after_move = chess.clone().play(&legal_move).unwrap();

                // Handle the child node rating
                if let RatingOrMove::Rating(mut child_node_rating) = (Node {
                    fen: Fen::from_position(pos_after_move, EnPassantMode::Legal),
                    layer_number: self.layer_number + 1,
                    previous_move: Some(legal_move.clone()),
                    previous_weight: current_node_weight,
                    previous_current_rating: current_rating,
                })
                .get_node_rating_or_move()
                {
                    // Start the optimisation
                    current_rating = if bot_turn {
                        current_rating.max(child_node_rating)
                    } else {
                        current_rating.min(child_node_rating)
                    };

                    match self.layer_number {
                        1 => {
                            if current_rating < self.previous_current_rating {
                                return RatingOrMove::Rating(-INFINITY);
                            }
                        }
                        case if case > 1 => {
                            if bot_turn {
                                if current_rating >= self.previous_current_rating {
                                    return RatingOrMove::Rating(INFINITY);
                                }
                            } else if current_rating <= self.previous_current_rating {
                                return RatingOrMove::Rating(-INFINITY);
                            }
                        }
                        _ => (),
                    }
                    // End the optimisation

                    if self.layer_number == 0 {
                        // Make a hashmap of { move: rating }
                        move_ratings.insert(legal_move, child_node_rating);
                    } else {
                        // Find the maximum or minimum rating depending on the turn
                        let child_node_rating_abs = child_node_rating.abs();

                        // If there's no checkmate or stalemate, the rating is corrected according
                        // to the number of moves of the bot and the opponent
                        if child_node_rating_abs != CHECKMATE_WEIGHT
                            && child_node_rating_abs != STALEMATE_WEIGHT
                            && child_node_rating_abs != INFINITY
                        {
                            child_node_rating += match self.layer_number {
                                1 => -(2 * moves_number), // The more moves the opponent has, the worse
                                2 => {
                                    // Needed during the opening for the bot to develop its pieces;
                                    // however, after the end of the opening, it may cause endless repetitive moves
                                    if opening_is_going {
                                        moves_number
                                    } else {
                                        0
                                    }
                                } // The more moves the bot has, the better
                                _ => 0,
                            }
                        } // Finish the correction

                        rating_to_return = if bot_turn {
                            // Chosing the best move for the bot
                            rating_to_return.max(child_node_rating)
                        } else {
                            // Chosing the best move for the opponent
                            rating_to_return.min(child_node_rating)
                        }
                    }
                }
            }

            // The final return phase
            if self.layer_number == 0 {
                // Retain the moves with the best final rating
                let max_rating_move = &move_ratings.iter().max_by_key(|entry| entry.1).unwrap().1;

                // Keep the elements with the best rating
                let mut filtered_move_ratings: HashMap<&Move, &i16> = HashMap::new();

                for move_rating in &move_ratings {
                    if &move_rating.1 == max_rating_move {
                        filtered_move_ratings.insert(move_rating.0, move_rating.1);
                    }
                }

                RatingOrMove::Move(
                    filtered_move_ratings
                        .keys()
                        .copied()
                        .next()
                        .unwrap()
                        .clone(),
                )
            } else {
                RatingOrMove::Rating(rating_to_return)
            }
        }
    }

    /// Get the weight of a specific piece.
    fn get_piece_weight(&self, piece: Role) -> i16 {
        match piece {
            Role::Pawn => PAWN_WEIGHT,
            Role::Knight => KNIGHT_WEIGHT,
            Role::Bishop => BISHOP_WEIGHT,
            Role::Queen => QUEEN_WEIGHT,
            Role::Rook => ROOK_WEIGHT,
            _ => 0,
        }
    }
}
