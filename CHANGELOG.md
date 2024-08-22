# Changelog

## 0.4.1

- `parseNotation` and `parseHexchess` now return `undefined` when input is invalid, rather than erroring

## 0.4.0

- Add `findKing` method
- Add `isCheckmate` method
- Improved typescript definitions

## 0.3.2

- Fix self-check logic on opponent's turn

## 0.3.1

- Fix self-check logic when king is moved piece

## 0.3.0

- Add `isThreatened` method
- Prohibit self-checking moves
- Fix en passant capture by non-pawns
- Remove regex parser to cut wasm binary from 1.6MB &rarr; 180KB

## 0.2.5

- Fix en passant capture not removing enemy piece
