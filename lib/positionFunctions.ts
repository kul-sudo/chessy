import { BISHOP_WEIGHT, KNIGHT_WEIGHT, PAWN_WEIGHT, QUEEN_WEIGHT, ROOK_WEIGHT } from './constants'
import * as ChessJS from 'chess.js'

const Chess = typeof ChessJS === 'function' ? ChessJS : ChessJS.Chess

const countInString = (string: string, toCount: string): number => {
  return string.split(toCount).length - 1
}

const getWeightOfPosition = (fen: string, botColor: 'w' | 'b'): number => {
  const chessInstance = new Chess(fen)

  if (chessInstance.in_checkmate()) {
    return botColor === chessInstance.turn() ? -Infinity : Infinity
  } else {
    const pieces = fen.substring(0, fen.indexOf(' '))

    const blackPiecesWeight = countInString(pieces, 'p') * PAWN_WEIGHT + countInString(pieces, 'r') * ROOK_WEIGHT + countInString(pieces, 'q') * QUEEN_WEIGHT + countInString(pieces, 'b') * BISHOP_WEIGHT + countInString(pieces, 'n') * KNIGHT_WEIGHT
    const whitePiecesWeight = countInString(pieces, 'P') * PAWN_WEIGHT + countInString(pieces, 'R') * ROOK_WEIGHT + countInString(pieces, 'Q') * QUEEN_WEIGHT + countInString(pieces, 'B') * BISHOP_WEIGHT + countInString(pieces, 'N') * KNIGHT_WEIGHT
    
    const subtracted = blackPiecesWeight - whitePiecesWeight

    return botColor === 'b' ? subtracted : -subtracted
  }
}

export { getWeightOfPosition }