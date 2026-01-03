# w7a2jkf

Converts Western-style Shogi game records to 
[JKF (JSON Kifu Format)](https://github.com/na2hiro/json-kifu-format), with an
eye toward a [KIF](https://lishogi.org/page/kif) output.

The [shogi-kif-converter](https://github.com/sugyan/shogi-kifu-converter) will
handle the translation to KIF from JKF.

## libraries

* [w7a](w7a) Parses `file.w7a` and converts that file to 
[JKF](https://github.com/na2hiro/json-kifu-format)

## Revisions

* 0.02, 2026-01-02: Reading `file.w7a` using LogicalGraphs
[book](https://github.com/logicalgraphs/crypto-n-rust/tree/main/src/libs/book) 
library.
* 0.01, 2026-01-01: Hand-convert part of a [w7a game 
record](https://www2.teu.ac.jp/gamelab/shogi/GAMES/54oi1.html) to JKF (comments
and two initial pawn-moves). Used 
[shogi-kifu-converter](https://github.com/sugyan/shogi-kifu-converter) to 
convert to KIF, which I then viewed in [lishogi](https://lishogi.org/pSUou8Lp).

