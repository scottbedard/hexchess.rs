import { describe, test, expect } from 'vitest'
import { index, San } from '../src'

describe('San', () => {
  test('success single digit promotion rank', () => {
    const san = San.from('a1b2')
    expect(san).toEqual({
      from: index('a1'),
      promotion: null,
      to: index('b2'),
    })
  })

  test('success single digit promotion rank', () => {
    const san = San.from('a1b2')
    expect(san).toEqual({
      from: index('a1'),
      promotion: null,
      to: index('b2'),
    })
  })

  test('success promotions', () => {
    expect(San.from('a5a6b')).toEqual({
      from: index('a5'),
      promotion: 'b',
      to: index('a6'),
    })

    expect(San.from('a5a6n')).toEqual({
      from: index('a5'),
      promotion: 'n',
      to: index('a6'),
    })

    expect(San.from('a5a6r')).toEqual({
      from: index('a5'),
      promotion: 'r',
      to: index('a6'),
    })

    expect(San.from('a5a6q')).toEqual({
      from: index('a5'),
      promotion: 'q',
      to: index('a6'),
    })
  })

  test('success two digit promotion rank', () => {
    expect(San.from('f10f11b')).toEqual({
      from: index('f10'),
      promotion: 'b',
      to: index('f11'),
    })

    expect(San.from('f10f11n')).toEqual({
      from: index('f10'),
      promotion: 'n',
      to: index('f11'),
    })

    expect(San.from('f10f11r')).toEqual({
      from: index('f10'),
      promotion: 'r',
      to: index('f11'),
    })

    expect(San.from('f10f11q')).toEqual({
      from: index('f10'),
      promotion: 'q',
      to: index('f11'),
    })
  })

  test('success to 10th rank', () => {
    expect(San.from('f9f10')).toEqual({
      from: index('f9'),
      promotion: null,
      to: index('f10'),
    })
  })

  test('empty string', () => {
    expect(() => San.from('')).toThrow()
  })

  test('missing rank', () => {
    expect(() => San.from('a')).toThrow()
  })

  test('missing third character', () => {
    expect(() => San.from('a1')).toThrow()
  })

  test('invalid second character', () => {
    expect(() => San.from('ax')).toThrow()
  })

  test('invalid to file', () => {
    expect(() => San.from('a1x')).toThrow()
    expect(() => San.from('a10x')).toThrow()
    expect(() => San.from('a11x')).toThrow()
  })

  test('invalid to second character', () => {
    expect(() => San.from('a1ax')).toThrow()
  })

  test('missing to file', () => {
    expect(() => San.from('a10')).toThrow()
  })

  test('missing to second character', () => {
    expect(() => San.from('f1f')).toThrow()
    expect(() => San.from('f10f')).toThrow()
    expect(() => San.from('f11f')).toThrow()
  })

  test('invalid to rank', () => {
    expect(() => San.from('a1f12')).toThrow()
  })

  test('invalid to second character', () => {
    expect(() => San.from('a1abc2')).toThrow()
  })

  test('invalid from position', () => {
    expect(() => San.from('a9a1')).toThrow()
  })

  test('invalid to position', () => {
    expect(() => San.from('a1a9')).toThrow()
  })

  test('invalid promotion character', () => {
    expect(() => San.from('f10f11x')).toThrow()
  })

  test('notation with invalid from and to', () => {
    expect(() => San.from('x1x2')).toThrow()
  })

  test('notation with identical from and to', () => {
    expect(() => San.from('a1a1')).toThrow()
  })

  test('post promotion character', () => {
    expect(() => San.from('f10f11qq')).toThrow()
  })

  test('invalid promotion position', () => {
    expect(() => San.from('f10f6q')).toThrow()
  })

  test('display string format', () => {
    expect(San.from('a1a2').toString()).toEqual('a1a2')
    expect(San.from('f10f11q').toString()).toEqual('f10f11q')
    expect(San.from('f10f11r').toString()).toEqual('f10f11r')
    expect(San.from('f10f11b').toString()).toEqual('f10f11b')
    expect(San.from('f10f11n').toString()).toEqual('f10f11n')
  })
})
