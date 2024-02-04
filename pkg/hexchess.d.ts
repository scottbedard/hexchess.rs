/* tslint:disable */
/* eslint-disable */
/**
* Execute notation on hexchess object
* @param {Hexchess} hexchess
* @param {Notation} notation
* @returns {Hexchess}
*/
export function applyNotation(hexchess: Hexchess, notation: Notation): Hexchess;
/**
* Create empty hexchess object
* @returns {Hexchess}
*/
export function createHexchess(): Hexchess;
/**
* Create hexchess object with initial position
* @returns {Hexchess}
*/
export function createHexchessInitial(): Hexchess;
/**
* Create hexchess object from string
* @param {string} fen
* @returns {Hexchess}
*/
export function parseHexchess(fen: string): Hexchess;
/**
* Create hexchess notation object from string
* @param {string} str
* @returns {Notation}
*/
export function parseNotation(str: string): Notation;
/**
* Stringify hexchess object
* @param {Hexchess} hexchess
* @returns {string}
*/
export function stringifyHexchess(hexchess: Hexchess): string;
export interface Board {
    a1: Piece | null;
    a2: Piece | null;
    a3: Piece | null;
    a4: Piece | null;
    a5: Piece | null;
    a6: Piece | null;
    b1: Piece | null;
    b2: Piece | null;
    b3: Piece | null;
    b4: Piece | null;
    b5: Piece | null;
    b6: Piece | null;
    b7: Piece | null;
    c1: Piece | null;
    c2: Piece | null;
    c3: Piece | null;
    c4: Piece | null;
    c5: Piece | null;
    c6: Piece | null;
    c7: Piece | null;
    c8: Piece | null;
    d1: Piece | null;
    d2: Piece | null;
    d3: Piece | null;
    d4: Piece | null;
    d5: Piece | null;
    d6: Piece | null;
    d7: Piece | null;
    d8: Piece | null;
    d9: Piece | null;
    e1: Piece | null;
    e2: Piece | null;
    e3: Piece | null;
    e4: Piece | null;
    e5: Piece | null;
    e6: Piece | null;
    e7: Piece | null;
    e8: Piece | null;
    e9: Piece | null;
    e10: Piece | null;
    f1: Piece | null;
    f2: Piece | null;
    f3: Piece | null;
    f4: Piece | null;
    f5: Piece | null;
    f6: Piece | null;
    f7: Piece | null;
    f8: Piece | null;
    f9: Piece | null;
    f10: Piece | null;
    f11: Piece | null;
    g1: Piece | null;
    g2: Piece | null;
    g3: Piece | null;
    g4: Piece | null;
    g5: Piece | null;
    g6: Piece | null;
    g7: Piece | null;
    g8: Piece | null;
    g9: Piece | null;
    g10: Piece | null;
    h1: Piece | null;
    h2: Piece | null;
    h3: Piece | null;
    h4: Piece | null;
    h5: Piece | null;
    h6: Piece | null;
    h7: Piece | null;
    h8: Piece | null;
    h9: Piece | null;
    i1: Piece | null;
    i2: Piece | null;
    i3: Piece | null;
    i4: Piece | null;
    i5: Piece | null;
    i6: Piece | null;
    i7: Piece | null;
    i8: Piece | null;
    k1: Piece | null;
    k2: Piece | null;
    k3: Piece | null;
    k4: Piece | null;
    k5: Piece | null;
    k6: Piece | null;
    k7: Piece | null;
    l1: Piece | null;
    l2: Piece | null;
    l3: Piece | null;
    l4: Piece | null;
    l5: Piece | null;
    l6: Piece | null;
}

export type Position = "a1" | "a2" | "a3" | "a4" | "a5" | "a6" | "b1" | "b2" | "b3" | "b4" | "b5" | "b6" | "b7" | "c1" | "c2" | "c3" | "c4" | "c5" | "c6" | "c7" | "c8" | "d1" | "d2" | "d3" | "d4" | "d5" | "d6" | "d7" | "d8" | "d9" | "e1" | "e2" | "e3" | "e4" | "e5" | "e6" | "e7" | "e8" | "e9" | "e10" | "f1" | "f2" | "f3" | "f4" | "f5" | "f6" | "f7" | "f8" | "f9" | "f10" | "f11" | "g1" | "g2" | "g3" | "g4" | "g5" | "g6" | "g7" | "g8" | "g9" | "g10" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "h7" | "h8" | "h9" | "i1" | "i2" | "i3" | "i4" | "i5" | "i6" | "i7" | "i8" | "k1" | "k2" | "k3" | "k4" | "k5" | "k6" | "k7" | "l1" | "l2" | "l3" | "l4" | "l5" | "l6";

export interface Hexchess {
    board: Board;
    enPassant: Position | null;
    fullmove: number;
    halfmove: number;
    turn: Color;
}

export type PromotionPiece = "n" | "b" | "r" | "q";

export type Piece = "P" | "N" | "B" | "R" | "Q" | "K" | "p" | "n" | "b" | "r" | "q" | "k";

export type Color = "w" | "b";

export interface Notation {
    from: Position;
    promotion: PromotionPiece | null;
    to: Position;
}


export function getTargets(hexchess: Hexchess, position: Position): Notation[];


