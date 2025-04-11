# [`@bedard/hexchess`](https://github.com/scottbedard/hexchess)

[![Build](https://github.com/scottbedard/hexchess/actions/workflows/build.yml/badge.svg)](https://github.com/scottbedard/hexchess/actions/workflows/build.yml)
[![Coverage](https://codecov.io/gh/scottbedard/hexchess/graph/badge.svg?token=uHmFqhQDps)](https://codecov.io/gh/scottbedard/hexchess)
[![NPM](https://img.shields.io/npm/v/%40bedard%2Fhexchess?logo=javascript&logoColor=%23f7df1e)](https://www.npmjs.com/package/@bedard/hexchess)
[![Crates.io](https://img.shields.io/crates/v/hexchess?logo=rust&logoColor=%23f74c00&label=cargo)](https://crates.io/crates/hexchess)
[![License](https://img.shields.io/badge/license-MIT-blue)](https://github.com/scottbedard/hexchess/blob/main/LICENSE)

A Rust / TypeScript library for [Gliński's hexagonal chess](https://en.wikipedia.org/wiki/Hexagonal_chess#Gli%C5%84ski's_hexagonal_chess), and the brain of [hexchess.club](https://hexchess.club).

<p align="center">
  <a href="https://hexchess.club">
    <img src="assets/hexchess.svg" width="500" />
  </a>
</p>

## Installation

Install this package via NPM.

Depending on your bundler, you may need plugins for [Web Assembly](https://developer.mozilla.org/en-US/docs/WebAssembly) and [top-level await](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/await#top_level_await).

```
npm install @bedard/hexchess

pnpm install @bedard/hexchess

yarn add @bedard/hexchess
```

## Basic usage

The `Hexchess` class is a deserialized version of [Forsyth–Edwards Notation](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation). It contains the board state, current turn, en passant, and move numbers. Since castling is not a part of hexagonal chess, that section is omitted. The board data is stored as an array of `Piece | null`, sorted in FEN-order.

To create a game at the starting position, use `Hexchess.init`. 

```js
import { Hexchess } from '@bedard/hexchess'

const hexchess = Hexchess.init()
```

`Hexchess` instances have the following shape. The `board` represents an array of position values, sorted in FEN-order.

```js
{
  board: [
    'b',  'q',  'b',  'k',  'n',  null, 'b',  null, 'n',  'r',
    null, null, null, null, null, 'r',  'p',  'p',  'p',  'p',
    'p',  'p',  'p',  'p',  'p',  null, null, null, null, null,
    null, null, null, null, null, null, null, null, null, null,
    null, 'P',  null, null, null, null, null, null, null, null,
    null, 'P',  null, 'P',  null, null, null, null, null, null,
    null, 'P',  null, 'B',  null, 'P',  null, null, null, null,
    null, 'P',  null, null, 'B',  null, null, 'P',  null, null,
    null, 'P',  'R',  'N',  'Q',  'B',  'K',  'N',  'R',  'P',
    null
  ],
  turn: 'w',
  ep: null,
  halfmove: 0,
  fullmove: 1
}
```

The following methods are available for interacting with the game.

#### `apply`

Apply a whitespace separated sequence of moves, or a single `San` object.

```ts
const hexchess = Hexchess.init()

hexchess.apply('g4g5 e7e6 f5f6 e6f6')

hexchess.apply(San.parse('g5f6'))

hexchess.toString() // 'b/qbk/n1b1n/r5r/ppppppppp/11/5PP4/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 b - 0 1'
```

#### `currentMoves`

Get all current legal moves.

```ts
const hexchess = new Hexchess('1/3/5/7/9/11/5P5/11/11/11/11 w - 0 1')

const moves = hexchess.currentMoves()

moves.map(String) // ['f5f6, 'f5f7', ...]
```

#### `movesFrom`

Get all legal moves from a position.

```js
const hexchess = Hexchess.init()

const moves = hexchess.movesFrom('f6')

moves.map(String) // ['f6f7']
```

#### `movesUnsafe`

Get all possible moves, including ones that result in self-check.

```ts
const hexchess = Hexchess.parse('1/3/5/7/4r4/5K5/11/11/11/11/11 w - 0 1')

const moves = hexchess.movesUnsafe()

moves.map(String) // ['f6f7', 'f6g7' ...]
```

#### `movesFromUnsafe`

Get all moves from a position, including ones that result in self-check.

#### `toString`

Serialize a `Hexchess` instance to string.

```ts
const hexchess = Hexchess.init()

hexchess.toString() // 'b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1'
```

## Function API

The raw WASM bindings are available for a more functional API. These work with objects matching the `Hexchess` and `San` structs. The API of these functions matches the Rust library API, [see here for more info &rarr;](https://github.com/scottbedard/hexchess/blob/main/src/lib.rs)

- `applyMove`
- `applySequence`
- `createHexchess`
- `currentMoves`
- `initHexchess`
- `movesFrom`
- `movesFromUnsafe`
- `parseHexchess`
- `parseSan`
- `stringifyHexchess`
- `stringifySan`

## License

[MIT](https://github.com/scottbedard/hexchess/blob/main/LICENSE)

Copyright (c) 2024-present, Scott Bedard
