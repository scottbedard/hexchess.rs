# `hexchess.rs`

[![Build](https://github.com/scottbedard/hexchess.rs/actions/workflows/build.yml/badge.svg)](https://github.com/scottbedard/hexchess.rs/actions/workflows/build.yml)
[![License](https://img.shields.io/badge/license-TBD-blue)](https://github.com/scottbedard/hexchess.rs/blob/main/LICENSE)

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

## WASM

A TypeScript web assembly module is available as `@bedard/hexchess`.

Most of these functions act on `Hexchess` objects, which are created using `parse`.

```ts
import { parse } from '@bedard/hexchess'

parse('b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1')

// { board: { ... }, en_passant, turn, fullmove, halfmove }
```

Use `stringify` to transform this object back to it's fen representation.

```ts
import { stringify } from '@bedard/hexchess'

stringify(hexchess)

// 'b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1'
```

Find legal moves from a position using `targets`

```ts
import { targets } from '@bedard/hexchess'

targets(hexchess, 'g4')

// [{ from, to, promotion }, ...]
```

To manually create `Notation` objects, use the `notation` function.

```ts
import { notation } from '@bedard/hexchess'

notation('e4e5')

// { from: 'e4', to: 'e5', promotion: null }
```

Use `apply` to execute a piece of notation. This function returns a new `Hexchess` object.

```ts
import { apply } from '@bedard/hexchess'

apply(hexchess, notation)

// { board: { ... }, en_passant, turn, fullmove, halfmove }
```

## License

TBD

Copyright (c) 2024-present, Scott Bedard
