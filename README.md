# `hexchess.rs`

<a href="https://github.com/scottbedard/hexchess.rs/actions/workflows/build.yml"><img src="https://github.com/scottbedard/hexchess.rs/actions/workflows/build.yml/badge.svg" /></a>
<a href="https://github.com/scottbedard/hexchess.rs/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" /></a>

A Rust library for [Gliński's hexagonal chess](https://en.wikipedia.org/wiki/Hexagonal_chess#Gli%C5%84ski's_hexagonal_chess), and the brain of [hexchess.club](https://hexchess.club)

<p align="center">
  <a href="https://hexchess.club">
    <img src="https://raw.githubusercontent.com/scottbedard/hexchess.rs/main/assets/hexchess.svg?token=GHSAT0AAAAAAB6TCUTJKYS3NBP6TEIW2DKOZN3FTWQ" width="500" />
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

A web assembly module can be accessed via `@bedard/hexchess`

```ts
import { parse } from '@bedard/hexchess'

const game = parse('b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1'')

// { board: { ... }, en_passant, turn, fullmove, halfmove }
```

## License

[MIT](https://github.com/scottbedard/hexchess.rs/tree/main?tab=MIT-1-ov-file#readme)

Copyright (c) 2024-present, Scott Bedard
