import type { NextPage } from 'next'
import type { Square } from 'chess.js'
import * as ChessJS from 'chess.js'
import { useCallback, useEffect, useState } from 'react'
import { Chessboard } from 'react-chessboard'
import { Move } from 'chess.js'
import { Button, Loader } from '@mantine/core'
import { invoke } from '@tauri-apps/api/tauri'
import { writeFile } from '@tauri-apps/api/fs'
import { PromotionPieceOption } from 'react-chessboard/dist/chessboard/types'

const SHOW_CHESSBOARD = true
const WRITE_TO_FILE = false
const BOT_VS_BOT = true

let gameHistory: string[] = []

const Chess = typeof ChessJS === 'function' ? ChessJS : ChessJS.Chess

const ChessboardPage: NextPage = () => {
  const [game, setGame] = useState(new Chess())
  const [moveFrom, setMoveFrom] = useState<Square | ''>()
  const [moveTo, setMoveTo] = useState<Square | null>(null)
  const [showPromotionDialog, setShowPromotionDialog] = useState(false)
  const [rightClickedSquares, setRightClickedSquares] = useState({})
  const [moveSquares, setMoveSquares] = useState({})
  const [optionSquares, setOptionSquares] = useState({})

  const [run, setRun] = useState(true)
  const [isLoading, setIsLoading] = useState(false)

  const makeMove = useCallback(
    (move: Move) => {
      const gameCopy = new Chess(game.fen())
      gameCopy.move(move)
      setGame(gameCopy)
    },
    [game]
  )
  const makeBotMove = useCallback(() => {
    invoke<Move>('get_move', {
      current_fen: game.fen()
    }).then(move => {
      makeMove(move)
      setIsLoading(false)
    })
  }, [game, makeMove])

  const resetGame = useCallback(() => {
    const gameCopy = new Chess(game.fen())
    gameCopy.reset()
    setGame(gameCopy)
  }, [game])

  const getMoveOptions = (square: Square): boolean => {
    const moves = game.moves({
      square,
      verbose: true
    })

    if (moves.length === 0) {
      setOptionSquares({})
      return false
    }

    const newSquares = {}

    moves.map(move => {
      newSquares[move.to] = {
        background:
          game.get(move.to) &&
          game.get(move.to).color !== game.get(square).color
            ? 'radial-gradient(circle, rgba(0,0,0,.1) 85%, transparent 85%)'
            : 'radial-gradient(circle, rgba(0,0,0,.1) 25%, transparent 25%)',
        borderRadius: '50%'
      }

      return move
    })

    newSquares[square] = {
      background: 'rgba(255, 255, 0, 0.4)'
    }

    setOptionSquares(newSquares)

    return true
  }

  const onSquareClick = async (square: Square) => {
    setRightClickedSquares({})

    // from square
    if (!moveFrom) {
      const hasMoveOptions = getMoveOptions(square)
      if (hasMoveOptions) {
        setMoveFrom(square)
      }

      return
    }

    if (!moveTo) {
      const moves = game.moves({
        verbose: true
      })

      const foundMove = moves.find(
        (m: Move) => m.from === moveFrom && m.to === square
      )

      if (!foundMove) {
        const hasMoveOptions = getMoveOptions(square)

        setMoveFrom(hasMoveOptions ? square : '')

        return
      }

      setMoveTo(square)

      if (
        (foundMove.color === 'w' &&
          foundMove.piece === 'p' &&
          square[1] === '8') ||
        (foundMove.color === 'b' &&
          foundMove.piece === 'p' &&
          square[1] === '1')
      ) {
        setShowPromotionDialog(true)

        return
      }

      const gameCopy = new Chess()
      gameCopy.load(game.fen())

      const move = gameCopy.move({
        from: moveFrom,
        to: square,
        promotion: 'q'
      })

      if (move === null) {
        const hasMoveOptions = getMoveOptions(square)

        if (hasMoveOptions) {
          setMoveFrom(square)
        }

        return
      }

      setGame(gameCopy)

      setIsLoading(true)

      setMoveFrom('')
      setMoveTo(null)
      setOptionSquares({})
    }
  }

  const onPromotionPieceSelect = (piece: PromotionPieceOption) => {
    if (piece) {
      makeMove({
        from: moveFrom as Square,
        to: moveTo,
        promotion: (piece[1].toLowerCase() ?? 'q') as ChessJS.PieceSymbol
      } as Move)

      setIsLoading(true)
    }

    setMoveFrom('')
    setMoveTo(null)
    setShowPromotionDialog(false)
    setOptionSquares({})

    return true
  }

  const onSquareRightClick = (square: Square) => {
    const colour = 'rgba(0, 0, 255, 0.4)'

    setRightClickedSquares({
      ...rightClickedSquares,
      [square]:
        rightClickedSquares[square] &&
        rightClickedSquares[square].backgroundColor === colour
          ? undefined
          : { backgroundColor: colour }
    })
  }

  useEffect(() => {
    if (game.isGameOver()) {
      if (WRITE_TO_FILE) {
        writeFile({
          path: '/home/user/Documents/chessy/data/game.json',
          contents: JSON.stringify(gameHistory)
        })
      }

      invoke('first_move')

      setTimeout(() => {
        gameHistory = []
        resetGame()
      }, 5000)
    }
  }, [game, resetGame])

  const game_turn = game.turn()

  useEffect(() => {
    if (!BOT_VS_BOT && game.turn() === 'b') {
      makeBotMove()
    }
  }, [game, game_turn, makeBotMove])

  useEffect(() => {
    if (WRITE_TO_FILE) {
      gameHistory.push(game.fen())
    }

    if (BOT_VS_BOT && run && !game.isGameOver()) {
      makeBotMove()
    }
  }, [game_turn, game, run, makeBotMove])

  return (
    <div
      style={{
        width: '70vw',
        maxWidth: '70vh',
        margin: '3rem auto'
      }}
    >
      {SHOW_CHESSBOARD && (
        <Chessboard
          animationDuration={200}
          arePiecesDraggable={false}
          position={game.fen()}
          onSquareClick={onSquareClick}
          onSquareRightClick={onSquareRightClick}
          onPromotionPieceSelect={onPromotionPieceSelect}
          customBoardStyle={{
            borderRadius: '4px',
            boxShadow: '0 2px 10px rgba(0, 0, 0, 0.5)',
            userSelect: 'none'
          }}
          customSquareStyles={{
            ...moveSquares,
            ...optionSquares,
            ...rightClickedSquares
          }}
          promotionToSquare={moveTo}
          showPromotionDialog={showPromotionDialog}
        />
      )}

      <Button
        disabled={isLoading}
        onClick={() => {
          resetGame()
          setMoveSquares({})
          setOptionSquares({})
          setRightClickedSquares({})
        }}
      >
        reset
      </Button>

      <Button
        disabled={isLoading}
        onClick={() => {
          setRun(!run)
        }}
      >
        {run ? 'Stop' : 'Run'}
      </Button>

      {isLoading && <Loader />}
    </div>
  )
}

export default ChessboardPage
