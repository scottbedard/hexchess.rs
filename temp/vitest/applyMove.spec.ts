import { expect, test } from 'vitest'
import { applyMove, initHexchess, stringifyHexchess } from '../pkg'

test('applyMove', () => {
  const hexchess = applyMove(initHexchess(), { from: 53, to: 42, promotion: null })

  expect(stringifyHexchess(hexchess)).toEqual('b/qbk/n1b1n/r5r/ppppppppp/11/5PP4/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 b - 0 1')
})
