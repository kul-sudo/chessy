import * as ChessJS from 'chess.js'
import { getWeightOfPosition } from './positionFunctions'
import { getKeysByMaxValue, getKeysByMinValue } from './dictionaryFunctions'

const Chess = typeof ChessJS === 'function' ? ChessJS : ChessJS.Chess

let botQueenMoved = false

const makeBotMove = (initialFen: string, playerQueenMoved: boolean): string => {
  // Step 1. Preparation.
  const botColor = initialFen.split(' ').at(1).at(0) as 'w' | 'b'

  // Step 2. Imitating the creation of a tree of moves for 3 steps
  const data = new Map<string, [number, number, number]>() // { the first move of the bot: the minimum weight of the final position for all moves of the opponent, the number of moves of the opponent, the minimum number of second moves of the bot for all moves of the opponent after this first move of the bot }
  // Infinity means that the bot can checkmate
  // -Infinity means that the bot can be checkmated by the opponent

  const chessInstance = new Chess(initialFen)

  if (chessInstance.in_checkmate()) {
    return undefined
  }

  const initialWeight = getWeightOfPosition(initialFen, botColor)

  const botLegalMoves = chessInstance.moves()

  for (const firstBotMove of botLegalMoves) {
    // console.log('firstBotMove', firstBotMove)

    chessInstance.load(initialFen)
    chessInstance.move(firstBotMove)

    if (chessInstance.in_checkmate()) {
      data.set(firstBotMove, [Infinity, -1, -1]) // -1 indicates that it doesn't matter what the element is equal to
    } else {
      const fenAfterFirstBotMove = chessInstance.fen()

      const opponentMoves = chessInstance.moves()

      const finalData = new Map<string, [number, number]>() // { the move of the opponent: the minimum final weight of the position after the second move of the bot, the number of moves of the bot that come in response }

      for (const opponentMove of opponentMoves) {
        // console.log('opponentMove', opponentMove)
        chessInstance.load(fenAfterFirstBotMove)
        chessInstance.move(opponentMove)

        if (chessInstance.in_checkmate()) {
          finalData.set(opponentMove, [-Infinity, 0])
        } else {
          const fenAfterOpponentMove = chessInstance.fen()

          const secondBotMoves = chessInstance.moves()

          let maxFinalWeight = -Infinity

          for (const secondBotMove of secondBotMoves) {
            chessInstance.load(fenAfterOpponentMove)
            chessInstance.move(secondBotMove)

            const fenAfterSecondBotMove = chessInstance.fen()

            maxFinalWeight = Math.max(
              maxFinalWeight,
              getWeightOfPosition(fenAfterSecondBotMove, botColor)
            )
            // console.log('i am here', secondBotMove, maxFinalWeight, botColor)
          }

          finalData.set(opponentMove, [maxFinalWeight, secondBotMoves.length])
        }
      }

      // Now handling finalData
      // console.log('firstBotMove=', firstBotMove)
      // console.log('finalData=', finalData)

      // Including the moves the opponent can make that result in the minimum final weight for the bot
      const opponentMovesMinWeight = getKeysByMinValue(
        finalData,
        opponentMoves,
        0
      )

      // Including the moves the opponent can make that result in the minimum number of second moves that the bot can make
      const opponentMovesMinWeightMinMoves = getKeysByMinValue(
        finalData,
        opponentMovesMinWeight,
        1
      )

      // Calculating the worst case for the bot
      // In 'finalData' all the values of the keys from opponentMovesMinWeightMinMoves are equal
      const minFinalWeight = finalData
        .get(opponentMovesMinWeightMinMoves.at(0))
        .at(0)
      const minSecondBotMoves = finalData
        .get(opponentMovesMinWeightMinMoves.at(0))
        .at(1)

      data.set(firstBotMove, [
        minFinalWeight,
        opponentMoves.length,
        minSecondBotMoves
      ])
    }
  }

  // Step 3. Making sure the bot does not make any moves with the queen until the user makes one.
  if (botQueenMoved === false && playerQueenMoved === false) {
    for (const firstBotMove of Array.from(data.keys())) {
      if (firstBotMove.toLowerCase().startsWith('q')) {
        chessInstance.load(initialFen)
        chessInstance.move(firstBotMove)

        const currentWeight = getWeightOfPosition(chessInstance.fen(), botColor)

        if (currentWeight <= initialWeight) {
          // Artificially reducing the weight of the position after the move of the queen
          const currentValue = data.get(firstBotMove)
          currentValue[0] -= 1000
          data.set(firstBotMove, currentValue)
        }
      }
    }
  }

  // Step 4. Including only the moves the bot can make that give the maximum weight in the final position.
  const bestWeightMoves = getKeysByMaxValue(data, botLegalMoves, 0)

  // Step 5. Including only the moves the bot can make that minimise the amount of moves the opponent can make.
  const bestWeightMinMoves = getKeysByMinValue(data, bestWeightMoves, 1)

  // Step 6. Including only the moves the bot can make that maximise the amount of moves the bot can make.
  const bestWeightMinMaxMoves = getKeysByMaxValue(data, bestWeightMinMoves, 2)

  // Step 7. Completion.
  const randomMove =
    bestWeightMinMaxMoves[
      Math.floor(Math.random() * bestWeightMinMaxMoves.length)
    ]

  if (
    botQueenMoved === false &&
    randomMove.toLocaleLowerCase().startsWith('q')
  ) {
    botQueenMoved = true
  }

  // console.log('randomMove=', randomMove)

  // console.log('bestWeightMoves=', bestWeightMoves)
  // console.log('bestWeightMinMoves=', bestWeightMinMoves)
  // console.log('bestWeightMinMaxMoves=', bestWeightMinMaxMoves)

  // console.log('data=', data)

  return randomMove
}

export { makeBotMove }
