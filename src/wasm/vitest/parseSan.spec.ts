import { expect, test } from 'vitest'
import { parseSan } from '../pkg'

test('parseSan', () => {
  const result = parseSan('g4g5')

  expect(result).toEqual({ from: 53, to: 42, promotion: null })
})
