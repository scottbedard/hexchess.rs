# [`hexchess.rs`](https://github.com/scottbedard/hexchess.rs)

[![Build](https://github.com/scottbedard/hexchess.rs/actions/workflows/build.yml/badge.svg)](https://github.com/scottbedard/hexchess.rs/actions/workflows/build.yml)
[![Coverage](https://codecov.io/gh/scottbedard/hexchess.rs/graph/badge.svg?token=uHmFqhQDps)](https://codecov.io/gh/scottbedard/hexchess.rs)
[![NPM](https://img.shields.io/npm/v/%40bedard%2Fhexchess?logo=javascript&logoColor=%23f7df1e)](https://www.npmjs.com/package/@bedard/hexchess)
[![Crates.io](https://img.shields.io/crates/v/hexchess?logo=rust&logoColor=%23f74c00&label=cargo)](https://crates.io/crates/hexchess)
[![License](https://img.shields.io/badge/license-MIT-blue)](https://github.com/scottbedard/hexchess.rs/blob/main/LICENSE)

> **Warning:** This project has migrated to pure TypeScript to remove Wasm environment requirements. The Rust version may be revived some day, but for now consider it deprecated. See [`scottbedard/hexchess.ts`](https://github.com/scottbedard/hexchess.ts) for the current version.

A Rust / TypeScript library for [Gliński's hexagonal chess](https://en.wikipedia.org/wiki/Hexagonal_chess#Gli%C5%84ski's_hexagonal_chess), and the brain of [hexchess.club](https://hexchess.club)

<p align="center">
  <a href="https://hexchess.club">
    <img src="assets/hexchess.svg" width="500" />
  </a>
</p>

## Basic usage

Execute `hexchess` to open the following command line interface.

```
Usage: hexchess <COMMAND>

Commands:
  apply        Apply sequence of moves to a position
  get-status   Get game status (w, b, stalemate, checkmate)
  get-targets  Get legal moves
  parse        Parse hexchess fen to JSON
  test-move    Test if a move is legal
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## TypeScript

A collection of wasm bindings available via `@bedard/hexchess`, listed below are the available methods.

> Note: Depending on the bundler, plugins may be required for [Web Assembly](https://developer.mozilla.org/en-US/docs/WebAssembly) and [top-level await](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/await#top_level_await).

#### `applyNotation`

Create a new `Hexchess` object and apply a single `Notation`.

```ts
import { applyNotation } from '@bedard/hexchess'

applyNotation(hexchess, notation)

// { board: { ... }, enPassant, turn, fullmove, halfmove }
```

#### `applySequence`

Create a new `Hexchess` object and apply a whitespace-separated sequence of moves. An error is thrown if a piece of notation is not valid or a move is illegal.

```ts
import { applySequence } from '@bedard/hexchess'

applySequence(hexchess, 'g4g5 e7e6')

// { board: { ... }, enPassant, turn, fullmove, halfmove }
```

#### `createHexchess`

Create an empty `Hexchess` object.

```ts
import { createHexchess } from '@bedard/hexchess'

createHexchess()

// { board: { ... }, enPassant, turn, fullmove, halfmove }
```

#### `createHexchessInitial`

Create a `Hexchess` object with the initial game state.

```ts
import { createHexchessInitial } from '@bedard/hexchess'

createHexchessInitial()

// { board: { ... }, enPassant, turn, fullmove, halfmove }
```

#### `findKing`

Find a player's king.

```ts
import { findKing } from '@bedard/hexchess'

findKing(hexchess, 'b')
```

#### `getColor`

Get the color of a piece.

```ts
import { getColor } from '@bedard/hexchess'

getColor('p') // 'b'
getColor('P') // 'w'
getColor('?') // null
```

#### `getPositionColor`

Get color of a piece by board position. If no piece is present, `null` will be returned.

```ts
import { getPositionColor } from '@bedard/hexchess'

getPositionColor(hexchess, 'f5') // 'w'
```

#### `getTargets`

Find all legal moves from a position and return an array of `Notation` objects.

```ts
import { getTargets } from '@bedard/hexchess'

targets(hexchess, 'g4')

// [{ from, to, promotion }, ...]
```

#### `isCheckmate`

Test if the board is in checkmate.

```ts
import { isCheckmate } from '@bedard/hexchess'

isCheckmate(hexchess) // true / false
```

#### `isThreatened`

Test if a position is threatened.

```ts
import { isThreatened } from '@bedard/hexchess'

isThreatened(hexchess, 'g10') // false
```

#### `parseHexchess`

Create a `Hexchess` object from it's string representation. If hexchess is invalid, `undefined` will be returned.

```ts
import { parseHexchess } from '@bedard/hexchess'

parseHexchess('b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1')

// { board: { ... }, enPassant, turn, fullmove, halfmove }
```

#### `parseNotation`

Create a `Notation` object from it's string representation. If notation is invalid, `undefined` will be returned.

```ts
import { parseNotation } from '@bedard/hexchess'

parseNotation('e4e5')

// { from: 'e4', to: 'e5', promotion: null }
```

#### `stringifyHexchess`

Convert a `Hexchess` object to it's string representation.

```ts
import { stringifyHexchess } from '@bedard/hexchess'

stringifyHexchess(hexchess)

// 'b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1'
```

## License

[MIT](https://github.com/scottbedard/hexchess.rs/blob/main/LICENSE)

Copyright (c) 2024-present, Scott Bedard
