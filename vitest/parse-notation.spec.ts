import { describe, expect, it } from 'vitest'
import { parseNotation } from '@bedard/hexchess'

describe('parseNotation', () => {
  it('parse notation', () => {
    const notation = parseNotation('g4g5')

    expect(notation).toEqual({
      from: 'g4',
      promotion: null,
      to: 'g5',
    })
  })

  it('error returns undefined', () => {
    const notation = parseNotation('whoops')

    expect(notation).toBeUndefined()
  })
})
