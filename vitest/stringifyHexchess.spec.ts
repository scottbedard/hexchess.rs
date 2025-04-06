import { describe, it, expect } from 'vitest'
import { initHexchess, initialPosition, stringifyHexchess } from '../pkg/hexchess'

describe('stringifyHexchess', () => {
  it('initial state', () => {
    const hexchess = initHexchess()
  
    expect(stringifyHexchess(hexchess)).toEqual(initialPosition)
  })
})