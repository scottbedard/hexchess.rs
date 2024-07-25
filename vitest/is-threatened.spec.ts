import {
  createHexchess,
  isThreatened,
} from '@bedard/hexchess'

import { describe, expect, it } from 'vitest'

describe('isThreatened', () => {
  it('threatened', () => {
    const hexchess = createHexchess()
    hexchess.board.f11 = 'K'
    hexchess.board.f6 = 'q'

    expect(isThreatened(hexchess, 'f11')).toBe(true)
  })

  it('not threatened', () => {
    const hexchess = createHexchess()
    hexchess.board.h9 = 'K'
    hexchess.board.f6 = 'r'

    expect(isThreatened(hexchess, 'h9')).toBe(false)
  })
})
