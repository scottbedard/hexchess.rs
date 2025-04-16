import { expect, test } from 'vitest'
import { stringifySan } from '../pkg'

test('stringifySan', () => {
  const result = stringifySan({ from: 53, to: 42, promotion: null })

  expect(result).toEqual('g4g5')
})
