# Tseitin Transformation CLI

Tseitin CLI enables you to transform boolean equations into Conjunctive Normal Form (CNF) with the [Tseitin-Transformation](https://en.wikipedia.org/wiki/Tseytin_transformation) via a command line interface.

The project is in early development atm.

## Usage 

Just run `cargo run` and console session will open. Parse a boolean expression to the console and per default the tseitin-transformed boolean expression will be outputted to `test.cnf` in [DIMACS](https://jix.github.io/varisat/manual/0.2.0/formats/dimacs.html) format.

Use `--help` for further options.


## Boolean Expresion

Currently following symboles are allowed to be used in the string-encoded boolean expression:
- `&` for and-operator
- `|` for or-operator
- `1`,`0` for `true` and `false`
- `(`,`)` for brackets
- Variable name, e.g. `aFc_2`
  - starts with `[a-z]`
  - followed by `[a-z]`,`[A-Z]`,`[0-9]` or `_`

## Ideas for Future

Following general features are to be added:
- `Expr` to string-encoded boolean expression for user-readability
- other operators (e.g. implication `=>`)
- log messages for debugging
- show relation between new variables from tseitin expression and the expression they represent
