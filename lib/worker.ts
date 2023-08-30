import _ from 'lodash'
import { makeBotMove } from './makeBotMove'

const ctx: Worker = self as any

ctx.addEventListener('message', event => {
  postMessage(makeBotMove(event.data[0], event.data[1]))
})

export default null as any