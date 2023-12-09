use shakmaty::{
    fen::{Epd, Fen},
    CastlingMode, Chess, EnPassantMode, Move, Position, Role,
};

use crate::{constants::*, mut_static::*};

/// Node struct.
pub struct Node {
    /// The FEN of the node.
    pub fen: String,

    /// The layer number the node is currently at.
    pub layer_number: i16,

    /// The move of the node edge (every egde connects 2 nodes).
    pub previous_move: Move,

    /// The weight of the predecessor node.
    pub previous_weight: i16,
}

impl Node {
    /// Get the rating of the current node (the final weight when both the bot and the opponent play in the best way possible);
    /// this weight may be adjusted according to the number of the legal moves that can me made by either the bot or the opponent.
    pub fn get_node_rating(self) -> String {
        // Create an instance of Chess with the current FEN
        let fen: Fen = self.fen.parse().unwrap();
        let chess: Chess = fen.clone().into_position(CastlingMode::Standard).unwrap();

        let bot_color = unsafe { BOT_COLOR };
        let bot_wants_stalemate = unsafe { BOT_WANTS_STALEMATE };

        let turn = chess.turn();

        // Handle a possible checkmate
        if chess.is_checkmate() {
            // Return the worst or best weight depending on who the checkmate has been performed by
            return if bot_color == turn {
                (-CHECKMATE_WEIGHT).to_string()
            } else {
                CHECKMATE_WEIGHT.to_string()
            };
        }

        // Handle a possible stalemate
        if chess.is_stalemate() {
            // Return the worst or best weight depending on whether the bot wants a stalemate;
            // however, a checkmate has a higher weight than a stalemate
            return if bot_wants_stalemate {
                STALEMATE_WEIGHT.to_string()
            } else {
                (-STALEMATE_WEIGHT).to_string()
            };
        }

        // Handling the other cases (everything all the way down)
        // If there has been no pawn promotion or capture, there's no need to recalculate the weight
        let mut current_node_weight = self.previous_weight;

        let is_capture = self.previous_move.is_capture();
        let is_promotion = self.previous_move.is_promotion();

        if is_capture || is_promotion {
            // If there has either been a capture or promotion, an
            // adjustment is done

            let coefficient = if turn == bot_color { -1 } else { 1 }; // Defines whether a capture or promotion is good for the bot depending on the turn/color

            if is_capture {
                // Adjust the weight as a result of the captured piece
                current_node_weight +=
                    self.get_piece_weight(self.previous_move.capture().unwrap()) * coefficient
            }

            if is_promotion {
                // Adjust the weight as a result of the promoted pawn
                current_node_weight +=
                    (self.get_piece_weight(self.previous_move.promotion().unwrap()) - PAWN_WEIGHT)
                        * coefficient;
            }
        } else {
            current_node_weight = self.previous_weight;
        }

        // If the top has been reached, it's time to return the current node weight, otherwise create the successor for this node
        if self.layer_number == TREE_HEIGHT {
            current_node_weight.to_string()
        } else {
            let turn_for_bot = bot_color == turn;
            let opening_is_going = unsafe { OPENING_IS_GOING };

            let mut result = if turn_for_bot { i16::MIN } else { i16::MAX };

            let legal_moves = chess.legal_moves();

            let moves_number = if self.layer_number <= 2 {
                legal_moves.len() as i16
            } else {
                i16::default()
            };

            for legal_move in &legal_moves {
                let pos_after_move = chess.clone().play(legal_move).unwrap();

                let mut node_rating = (Node {
                    fen: Epd::from_position(pos_after_move, EnPassantMode::Legal).to_string(),
                    layer_number: self.layer_number + 1,
                    previous_move: legal_move.clone(),
                    previous_weight: current_node_weight,
                })
                .get_node_rating()
                .parse::<i16>()
                .unwrap();

                let node_rating_abs = node_rating.abs();

                if node_rating_abs != CHECKMATE_WEIGHT && node_rating_abs != STALEMATE_WEIGHT {
                    node_rating += match self.layer_number {
                        1 => -(2 * moves_number), // The more moves the opponent has, the worse
                        2 => {
                            // Needed during the opening for the bot to develop its pieces;
                            // however, after the end of the opening, it causes endless repetitive moves
                            if opening_is_going {
                                moves_number
                            } else {
                                0
                            }
                        } // The more moves the bot has, the better
                        _ => 0,
                    };
                }

                result = if turn_for_bot {
                    // Chosing the best move for the bot
                    result.max(node_rating)
                } else {
                    // Chosing the best move for the opponent
                    result.min(node_rating)
                }
            }

            result.to_string()
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
