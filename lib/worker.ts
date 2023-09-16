import { makeBotMove } from './makeBotMove'

addEventListener('message', (event) => {
  postMessage(makeBotMove(event.data[0], event.data[1]))
})