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

## Installation

Install this package via NPM.

Depending on your bundler, you may need plugins for [Web Assembly](https://developer.mozilla.org/en-US/docs/WebAssembly) and [top-level await](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/await#top_level_await).

```
# npm
npm install @bedard/hexchess

# pnpm
pnpm install @bedard/hexchess

# yarn
yarn add @bedard/hexchess
```

## Basic usage

The command line interface has not been migrated yet. Check back later.

## TypeScript

A collection of wasm bindings available via `@bedard/hexchess`, listed below are the available methods.

#### `applySequence`

Apply a whitespace separated sequence of move to a `Hexchess` object.

```ts
import { applySequence } from '@bedard/hexchess'

applySequence(hexchess, 'g4g6 f7g6 f5f7 g6f6') // { board: [ ... ], ep, turn, fullmove, halfmove }
```

#### `createHexchess`

Create a blank `Hexchess` object.

```ts
import { createHexchess } from '@bedard/hexchess'

createHexchess() // { board: [ ... ], ep, turn, fullmove, halfmove }
```

#### `currentMoves`

Get current legal moves.

```ts
import { currentMoves } from '@bedard/hexchess'

currentMoves(hexchess) // [{ from, to, promotion }, ...]
```

#### `initHexchess`

Create `Hexchess` object at the initial position.

```ts
import { initHexchess } from '@bedard/hexchess'

initHexchess() // { board: [ ... ], ep, turn, fullmove, halfmove }
```

#### `parseHexchess`

Parse `Hexchess` object from [Forsyth–Edwards Notation](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation).

```ts
import { parseHexchess } from '@bedard/hexchess'

parseHexchess('b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1') // { board, turn, ep, halfmove, fullmove }
```

#### `stringifyHexchess`

Convert `Hexchess` object to string using [Forsyth–Edwards Notation](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation).

```ts
import { stringifyHexchess } from '@bedard/hexchess'

stringifyHexchess(hexchess) // 'b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1'
```

### `stringifySan`

Convert `San` object to string.

```ts
import { stringifySan } from '@bedard/hexchess'

stringifySan({ from: 53, to: 42, promotion: null }) // 'g4g5'
```

## License

[MIT](https://github.com/scottbedard/hexchess.rs/blob/main/LICENSE)

Copyright (c) 2024-present, Scott Bedard
