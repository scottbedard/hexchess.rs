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

  /** convert san to string */
  toString() {
    return `${positions[this.from]}${positions[this.to]}${this.promotion ?? ''}`
  }
}
