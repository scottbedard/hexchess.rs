# `hexchess.rs`

[![Build](https://github.com/scottbedard/hexchess.rs/actions/workflows/build.yml/badge.svg)](https://github.com/scottbedard/hexchess.rs/actions/workflows/build.yml)
[![Coverage](https://codecov.io/gh/scottbedard/hexchess.rs/graph/badge.svg?token=uHmFqhQDps)](https://codecov.io/gh/scottbedard/hexchess.rs)
[![License](https://img.shields.io/badge/license-MIT-blue)](https://github.com/scottbedard/hexchess.rs/blob/main/LICENSE)

A Rust / TypeScript library for [Gliński's hexagonal chess](https://en.wikipedia.org/wiki/Hexagonal_chess#Gli%C5%84ski's_hexagonal_chess), and the brain of [hexchess.club](https://hexchess.club)

<p align="center">
  <a href="https://hexchess.club">
    <img src="assets/hexchess.svg" width="500" />
  </a>
</p>

## Basic usage

Execute `hexchess` to open the following command line interface. More documentation to come.

```
Usage: hexchess <COMMAND>

Commands:
  apply    Apply notation to game
  get      Get piece value at position
  parse    Parse hexchess to JSON
  targets  List target moves from a position
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## TypeScript

A collection of wasm bindings available via `@bedard/hexchess`, listed below are the available methods.

> Note: Depending on the bundler, plugins may be required for [Web Assembly](https://developer.mozilla.org/en-US/docs/WebAssembly) and [top-level await](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/await#top_level_await).

#### `applyNotation`

Create a new `Hexchess` object after applying a piece of `Notation`.

```ts
import { applyNotation } from '@bedard/hexchess'

applyNotation(hexchess, notation)

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

#### `getTargets`

Find all legal moves from a position and return the resulting array of `Notation` objects.

```ts
import { getTargets } from '@bedard/hexchess'

targets(hexchess, 'g4')

// [{ from, to, promotion }, ...]
```

#### `parseHexchess`

Create a `Hexchess` object from it's string representation.

```ts
import { parseHexchess } from '@bedard/hexchess'

parseHexchess('b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1')

// { board: { ... }, enPassant, turn, fullmove, halfmove }
```

#### `parseNotation`

Create a `Notation` object from it's string representation.

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
