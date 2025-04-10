import { describe, expect, test } from 'vitest'
import { San } from '../pkg'

describe('San', () => {
  test('constructor', () => {
    const san = new San({ from: 51, to: 40 })

    expect(san.from).toBe(51)
    expect(san.to).toBe(40)
    expect(san.promotion).toBe(null)
  })

  test('parse', () => {
    const san = San.parse('e4e5')

    expect(san.from).toBe(51)
    expect(san.to).toBe(40)
    expect(san.promotion).toBe(null)
  })

  test('toString', () => {
    const san = San.parse('e4e5')

    expect(san.toString()).toBe('e4e5')
  })
})
