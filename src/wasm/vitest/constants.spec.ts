import { describe, expect, test } from 'vitest'
import { initialPosition, positions } from '../pkg'

describe('constants', () => {
  test('initialPosition', () => {
    expect(typeof initialPosition).toBe('string')
  })

  test('positions', () => {
    expect(positions).toBeInstanceOf(Array)
    expect(positions.length).toBe(91)
  })
})
