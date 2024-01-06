use std::collections::HashMap;

use rand::{seq::SliceRandom, thread_rng};
use shakmaty::{fen::Fen, san::San, CastlingMode, Chess, EnPassantMode, Position, Role};

use crate::{constants::*, mut_static::*, utils::RatingOrMove};

/// Node struct.
pub struct Node {
    /// The FEN of the node.
    pub fen: Fen,

    /// The layer number the node is currently at.
    pub layer_number: i16,

    /// The weight of the current node.
    pub weight: i16,

    /// The current value of `b.current_rating` which is the parent of `a`; the value of
    /// `b.current_rating` at the moment when `a` was created. Used for optimisation.
    pub previous_current_rating: i16,
}

impl Node {
    /// Get the rating of the current node (the final weight when both the bot and the opponent play in the best way possible);
    /// this weight may be adjusted according to the number of the legal moves that can me made by either the bot or the opponent.
    pub fn get_node_rating_or_move(self) -> RatingOrMove {
        let thread_rng_ = &mut thread_rng();
        let stalemate_weight = unsafe { STALEMATE_WEIGHT };

        // Create an instance of Chess with the current FEN
        let chess: Chess = self
            .fen
            .clone()
            .into_position(CastlingMode::Standard)
            .unwrap();

        let bot_turn = chess.turn() == unsafe { BOT_COLOR };

        unsafe {
            if ONE_NODE_HANDLE_TIME == -1.0 {
                // println!("NODES_NUMBER = {:?}", NODES_NUMBER);
                NODES_NUMBER += 1
            }
        }

        if self.layer_number > 0 {
            // JS doesn't call Rust if there's a checkmate or stalemate

            // Handle a possible checkmate
            if chess.is_checkmate() {
                let checkmate_weight_for_this_layer = CHECKMATE_WEIGHT - self.layer_number;
                // Return the worst or best weight depending on who the checkmate has been performed by
                return RatingOrMove::Rating(if bot_turn {
                    -checkmate_weight_for_this_layer
                } else {
                    checkmate_weight_for_this_layer
                });
            }

            // Handle a possible stalemate
            if chess.is_stalemate() {
                let stalemate_weight_for_this_layer = stalemate_weight - self.layer_number;
                // Return the worst or best weight depending on whether the bot wants a stalemate;
                // however, a checkmate has a higher weight than a stalemate
                return RatingOrMove::Rating(if unsafe { BOT_WANTS_STALEMATE } {
                    stalemate_weight_for_this_layer
                } else {
                    -stalemate_weight_for_this_layer
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

        if self.layer_number == unsafe { TREE_HEIGHT } {
            // If the top has been reached, it's time to return the current node weight,
            // otherwise create the successor for this node
            RatingOrMove::Rating(self.weight)
        } else {
            let opening_is_going = unsafe { OPENING_IS_GOING };

            let mut rating_to_return = if bot_turn { -INFINITY } else { INFINITY };

            let mut legal_moves_shuffled = legal_moves;
            legal_moves_shuffled.shuffle(thread_rng_);

            let coefficient = if bot_turn { -1 } else { 1 }; // Defines whether a capture or promotion is good for the bot depending on the turn/color

            for legal_move in legal_moves_shuffled {
                let pos_after_move = chess.clone().play(&legal_move).unwrap();

                // Start handling the weight of the child node
                let mut child_node_weight = self.weight;

                // If there has either been a capture or promotion, an adjustment is done
                if legal_move.is_capture() {
                    // Adjust the weight as a result of the captured piece
                    child_node_weight +=
                        self.get_piece_weight(legal_move.capture().unwrap()) * coefficient
                }

                if legal_move.is_promotion() {
                    // Adjust the weight as a result of the promoted pawn
                    child_node_weight += (self.get_piece_weight(legal_move.promotion().unwrap())
                        - PAWN_WEIGHT)
                        * coefficient;
                }
                // Finish handling the weight

                // Handle the child node rating
                if let RatingOrMove::Rating(mut child_node_rating) = (Node {
                    fen: Fen::from_position(pos_after_move, EnPassantMode::Legal),
                    layer_number: self.layer_number + 1,
                    weight: child_node_weight,
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
                        // If there's no checkmate or stalemate, the rating is corrected according
                        // to the number of moves of the bot and the opponent
                        if child_node_rating.abs() < stalemate_weight {
                            child_node_rating += match self.layer_number {
                                1 => -(2 * moves_number), // The more moves the opponent has, the worse
                                2 if opening_is_going => {
                                    // Needed during the opening for the bot to develop its pieces;
                                    // however, after the end of the opening, it may cause endless repetitive moves
                                    moves_number
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
                // Keep the elements with the best rating
                let max_rating = move_ratings.values().max().cloned().unwrap();
                let mut tmp = HashMap::new();
                for entry in move_ratings.clone().into_iter() {
                    tmp.insert(San::from_move(&chess, &entry.0).to_string(), entry.1);
                }
                println!("{:?}", tmp);
                println!("{:}", "-".repeat(10));
                move_ratings.retain(|_, v| *v == max_rating);

                RatingOrMove::Move(move_ratings.keys().next().unwrap().clone())
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
