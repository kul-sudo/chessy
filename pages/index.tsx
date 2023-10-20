import type { FC } from 'react'
import type { Square } from 'chess.js'
import { useCallback, useEffect, useRef, useState } from 'react'
import * as ChessJS from 'chess.js'
import { Chessboard } from 'react-chessboard'
import { PromotionPieceOption } from 'react-chessboard/dist/chessboard/types'
import { Button, Loader } from '@mantine/core'

const SHOW_CHESSBOARD = true
let playerQueenMoved = false

const Chess = typeof ChessJS === 'function' ? ChessJS : ChessJS.Chess

const boardWrapper = {
  width: '70vw',
  maxWidth: '70vh',
  margin: '3rem auto'
}

type SafeGameMutateProps = (game: ChessJS.ChessInstance) => undefined

const ChessboardPage: FC = () => {
  const [game, setGame] = useState(new Chess())
  const [moveFrom, setMoveFrom] = useState<Square | ''>()
  const [moveTo, setMoveTo] = useState<Square | null>(null)
  const [showPromotionDialog, setShowPromotionDialog] = useState(false)
  const [rightClickedSquares, setRightClickedSquares] = useState({})
  const [moveSquares, setMoveSquares] = useState({})
  const [optionSquares, setOptionSquares] = useState({})

  const [isLoading, setIsLoading] = useState(false)

  const workerRef = useRef<Worker>()

  useEffect(() => {
    workerRef.current = new Worker(new URL('../lib/worker.ts', import.meta.url))

    workerRef.current.onmessage = event => {
      makeBotMoveMain(event.data)
      setIsLoading(false)
    }

    return () => {
      workerRef.current.terminate()
    }
  }, [])

  const handleWorker = useCallback(
    (toPost: { fen: string; playerQueenMoved: boolean }) => {
      workerRef.current.postMessage([toPost.fen, toPost.playerQueenMoved])
    },
    []
  )

  const safeGameMutate = (modify: SafeGameMutateProps) => {
    setGame((g: ChessJS.ChessInstance) => {
      const update = { ...g }
      modify(update)
      return update
    })
  }

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

  const makeBotMoveMain = (botMoveToMake: string) => {
    if (game.in_stalemate()) {
      console.log('draw')

      setTimeout(() => {
        safeGameMutate(game => {
          game.reset()
        })
      }, 3000)

      setMoveSquares({})
      setOptionSquares({})
      setRightClickedSquares({})

      return
    }

    if (game.in_checkmate()) {
      console.log('checkmate')
      console.log(`${game.turn()} lost`)

      setTimeout(() => {
        safeGameMutate(game => {
          game.reset()
        })
      }, 3000)

      setMoveSquares({})
      setOptionSquares({})
      setRightClickedSquares({})

      return
    }

    if (game.in_draw()) {
      console.log('draw')

      setTimeout(() => {
        safeGameMutate(game => {
          game.reset()
        })
      }, 3000)

      setMoveSquares({})
      setOptionSquares({})
      setRightClickedSquares({})

      return
    }

    safeGameMutate(game => {
      const botMove = botMoveToMake

      game.move(botMove)
    })

    if (game.in_stalemate()) {
      console.log('draw')

      setTimeout(() => {
        safeGameMutate(game => {
          game.reset()
        })
      }, 3000)

      setMoveSquares({})
      setOptionSquares({})
      setRightClickedSquares({})

      return
    }

    if (game.in_checkmate()) {
      console.log('checkmate')
      console.log(`${game.turn()} lost`)

      setTimeout(() => {
        safeGameMutate(game => {
          game.reset()
        })
      }, 3000)

      setMoveSquares({})
      setOptionSquares({})
      setRightClickedSquares({})

      return
    }

    if (game.in_draw()) {
      console.log('draw')

      setTimeout(() => {
        safeGameMutate(game => {
          game.reset()
        })
      }, 3000)

      setMoveSquares({})
      setOptionSquares({})
      setRightClickedSquares({})

      return
    }
  }

  const onSquareClick = (square: Square) => {
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
        moveFrom,
        verbose: true
      } as any)

      const foundMove = moves.find(
        (m: ChessJS.Move) => m.from === moveFrom && m.to === square
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

      const gameCopy = { ...game }

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

      if (foundMove.piece.toLocaleLowerCase() === 'q') {
        playerQueenMoved = true
      }

      setGame(gameCopy)

      setIsLoading(true)

      handleWorker({ fen: game.fen(), playerQueenMoved: playerQueenMoved })

      setMoveFrom('')
      setMoveTo(null)
      setOptionSquares({})

      return
    }
  }

  const onPromotionPieceSelect = (piece: PromotionPieceOption) => {
    if (piece) {
      const gameCopy = { ...game }

      gameCopy.move({
        from: moveFrom as Square,
        to: moveTo,
        promotion: (piece[1].toLowerCase() ?? 'q') as 'n' | 'b' | 'r' | 'q'
      })

      setIsLoading(true)

      handleWorker({ fen: game.fen(), playerQueenMoved: playerQueenMoved })
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

  return (
    <div style={boardWrapper}>
      {SHOW_CHESSBOARD && (
        <Chessboard
          id="ClickToMove"
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
          safeGameMutate(game => {
            game.reset()
          })
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
          safeGameMutate(game => {
            game.undo()
          })
          setMoveSquares({})
          setOptionSquares({})
          setRightClickedSquares({})
        }}
      >
        undo
      </Button>

      {isLoading && <Loader />}
    </div>
  )
}

export default ChessboardPage
