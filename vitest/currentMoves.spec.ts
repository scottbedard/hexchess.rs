import { describe, it, expect } from 'vitest'
import { currentMoves, initHexchess } from '../pkg/hexchess'

describe('currentMoves', () => {
  it('initial state', () => {
    const sans = currentMoves(initHexchess())
    
    expect(sans.length).toBe(51)
    expect(sans[0]).toEqual({ from: 41, promotion: null, to: 30 })
  })
})