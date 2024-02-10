use std::{
    cmp::{max, min},
    collections::HashMap,
};

use rand::{seq::SliceRandom, thread_rng};
use shakmaty::{fen::Fen, uci::Uci, CastlingMode, Chess, Color, EnPassantMode, Position, Role};

use crate::{
    constants::*, correct_rating, get_only_position, get_piece_weight, handle_checkmate_or_draw,
    mut_static::*, opening_book::opening_book, optimise, queen_or_king_first_move_handle,
    utils::RatingOrMove,
};

pub struct Node {
    /// The FEN of the node.
    pub fen: Fen,

    /// The layer number the node is currently at.
    pub layer_number: i32,

    /// The weight of the current node.
    pub weight: i32,

    /// The current value of `parent.current_rating`, which is the parent of the current node,
    /// at the moment when the current node was created. Used for the optimisation.
    pub previous_current_rating: i32,
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

        let layer_is_0 = self.layer_number == 0;
        if layer_is_0 {
            match opening_book(
                self.fen
                    .clone()
                    .to_string()
                    .rsplitn(3, ' ')
                    .collect::<Vec<&str>>()
                    .last()
                    .unwrap(),
            ) {
                case if case.is_empty() => unsafe {
                    match BOT_COLOR {
                        Color::White => LAST_MOVE_FROM_BOOK_W = false,
                        Color::Black => LAST_MOVE_FROM_BOOK_B = false,
                    }
                },
                case => {
                    unsafe {
                        match BOT_COLOR {
                            Color::White => LAST_MOVE_FROM_BOOK_W = true,
                            Color::Black => LAST_MOVE_FROM_BOOK_B = true,
                        }
                    }
                    return RatingOrMove::Move(
                        case.choose(&mut thread_rng())
                            .unwrap()
                            .parse::<Uci>()
                            .unwrap()
                            .to_move(&chess)
                            .unwrap(),
                    );
                }
            }
        }

        let bot_turn = chess.turn() == unsafe { BOT_COLOR };

        unsafe { NODES_NUMBER += 1 }

        if self.layer_number > 0 {
            // JS doesn't call Rust if there's a checkmate or stalemate
            handle_checkmate_or_draw!(chess, bot_turn, self.layer_number);
        }

        // Handling all the other cases (everything all the way down)
        let infinity_by_bot_turn = if bot_turn { -INFINITY } else { INFINITY };
        let mut current_rating = infinity_by_bot_turn; // Needed for
                                                       // optimisation

        let legal_moves = chess.legal_moves();
        let moves_number = legal_moves.len() as i32;
        let mut move_ratings = HashMap::with_capacity(moves_number as usize); // { move: rating of the move };
                                                                              // needed only for the root (layer number = 0)

        if self.layer_number == unsafe { TREE_HEIGHT } {
            // If the top has been reached, it's time to return the current node weight,
            // otherwise create the successor for this node
            RatingOrMove::Rating(self.weight)
        } else {
            let opening_is_going = unsafe { OPENING_IS_GOING };

            let mut rating_to_return = infinity_by_bot_turn;

            let coefficient = if bot_turn { 1 } else { -1 }; // Defines whether a capture or promotion is good for the bot depending on the turn/color

            for legal_move in {
                let mut legal_moves_shuffled = legal_moves;
                legal_moves_shuffled.shuffle(&mut thread_rng());
                legal_moves_shuffled
            } {
                let temp_chess = {
                    let mut chess_clone = chess.clone();
                    chess_clone.play_unchecked(&legal_move);
                    chess_clone
                };

                queen_or_king_first_move_handle!(layer_is_0, legal_move);

                // Start handling the weight of the child node
                let mut child_node_weight = self.weight;

                // If there has either been a capture or promotion, an adjustment is done
                if legal_move.is_capture() {
                    // Adjust the weight as a result of the captured piece
                    child_node_weight +=
                        get_piece_weight!(legal_move.capture().unwrap()) * coefficient
                }

                if legal_move.is_promotion() {
                    // Adjust the weight as a result of the promoted pawn
                    child_node_weight += (get_piece_weight!(legal_move.promotion().unwrap())
                        - PAWN_WEIGHT)
                        * coefficient;
                }
                // Finish handling the weight

                // Handle the child node rating
                let incremented_layer_number = self.layer_number + 1;

                if let RatingOrMove::Rating(child_node_rating) = (Node {
                    fen: Fen::from_position(temp_chess, EnPassantMode::Legal),
                    layer_number: incremented_layer_number,
                    weight: child_node_weight,
                    previous_current_rating: current_rating,
                })
                .get_node_rating_or_move()
                {
                    if child_node_rating.abs()
                        == CHECKMATE_WEIGHT_STARTING_POINT - incremented_layer_number
                    {
                        // If a checkmate legal move has been found, there are obviously no
                        // "better" moves
                        return if layer_is_0 {
                            RatingOrMove::Move(legal_move)
                        } else {
                            RatingOrMove::Rating(child_node_rating)
                        };
                    }

                    optimise!(
                        current_rating,
                        child_node_rating,
                        bot_turn,
                        self.previous_current_rating,
                        self.layer_number
                    );

                    if layer_is_0 {
                        // Make a hashmap of { move: rating }
                        move_ratings.insert(legal_move, child_node_rating);
                    } else {
                        // correct_rating!(
                        //     self.fen,
                        //     unsafe { BOT_COLOR },
                        //     child_node_rating,
                        //     opening_is_going,
                        //     moves_number,
                        //     self.layer_number
                        // );

                        rating_to_return =
                            (if bot_turn { max } else { min })(rating_to_return, child_node_rating);
                    }
                }
            }

            // The final return phase
            if layer_is_0 {
                // Keep the elements with the best rating
                let max_rating = move_ratings.values().max().cloned().unwrap();
                move_ratings.retain(|_, v| *v == max_rating);

                RatingOrMove::Move(move_ratings.keys().nth(0).unwrap().clone())
            } else {
                RatingOrMove::Rating({
                    correct_rating!(
                        rating_to_return,
                        self.fen,
                        unsafe { BOT_COLOR },
                        opening_is_going,
                        moves_number,
                        self.layer_number
                    );

                    rating_to_return
                })
            }
        }
    }
}
