import { expect, test } from 'vitest'
import { applySequence, initHexchess, stringifyHexchess } from '../../../pkg/hexchess'

test('applySequence', () => {
  const hexchess = applySequence(initHexchess(), 'g4g6 f7g6 f5f7 g6f6')

  expect(stringifyHexchess(hexchess)).toEqual('b/qbk/n1b1n/r5r/pppp1pppp/5p5/11/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 3')
})
