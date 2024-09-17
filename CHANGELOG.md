# Changelog

## 0.10.0 (not published)

- Rename `apply-sequence` command to `apply`
- Rename `check-move` command to `test-move`

## 0.9.0

- Add `check-move` command

## 0.8.0

- Add `all-targets` command

## 0.7.0

- Add `get-targets` command
- Remove `get` command

## 0.6.0

- Cargo library published to crates.io as [`hexchess`](https://crates.io/crates/hexchess)

## 0.5.0

- Add `applySequence` method
- Prevent `apply` from being called with a valid but illegal move
- To simplify type definitions, `findKing`, `getPieceColor` and `getPositionColor` will now return `undefined` instead of `null`

## 0.4.3

- Error when characters exist after the promotion notation

## 0.4.2

- Fixed notation parsing edge cases

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
