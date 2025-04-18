import { error } from './utils'
import { index, positions } from './index'
import { Position, PromotionPiece } from './types'

export class San {
  public from: number = 0

  public promotion: PromotionPiece | null = null

  public to: number = 0

  /** create san instance */
  constructor(obj: {
    from: number | Position
    to: number | Position
    promotion?: PromotionPiece | null
  }) {
    this.from = typeof obj.from === 'string' ? index(obj.from) : obj.from
    this.to = typeof obj.to === 'string' ? index(obj.to) : obj.to
    this.promotion = obj.promotion ?? null
  }

  static from(source: string) {
    const from = positions.find(position => source.startsWith(position))

    if (!from) {
      error(`invalid san from: ${from}`)
    }

    const tail = source.slice(from!.length)
    const to = positions.find(position => tail.startsWith(position))

    if (!to) {
      error(`invalid san to: ${to}`)
    }

    if (from === to) {
      error('invalid san: from and to are the same')
    }

    let promotion: PromotionPiece | null = null

    if (source.length > from!.length + to!.length) {
      const last = source[source.length - 1]

      if (['n', 'r', 'b', 'q'].includes(last)) {
        if (isPromotionPosition(index(to))) {
          promotion = last as PromotionPiece
        } else {
          error(`invalid san promotion: ${last}`)
        }
      } else {
        error(`invalid san promotion: ${last}`)
      }
    }

    if (from.length + to.length + (promotion ? 1 : 0) !== source.length) {
      error(`invalid san: ${source}`)
    }

    return new San({
      from: index(from),
      promotion,
      to: index(to),
    })
  }

  /** convert san to string */
  toString() {
    return `${positions[this.from]}${positions[this.to]}${this.promotion ?? ''}`
  }
}

/** check if position is a promotion position */
const isPromotionPosition = (position: number) => [
  80, // a1
  81, // b1
  82, // c1
  83, // d1
  84, // e1
  85, // f1
  86, // g1
  87, // h1
  88, // i1
  89, // k1
  90, // l1,
  25, // a6
  16, // b7
  9, // c8
  4, // d9
  1, // e10
  0, // f11
  3, // g10
  8, // h9
  15, // i8
  24, // k7
  35, // l6
].includes(position)
