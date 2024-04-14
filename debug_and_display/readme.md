## Debug & Display

- Types which "want" to be <b>printable</b> `Debug` or `Display` traits must be
  implemented

- Automatic implementations are only provided for types in the standard library

- The `Debug` trait can be implemented simply by using <b>derivable trait</b>

- The `Display` trait must be <b>manually</b> implemented

### println! and format!

Printing is handled by a series of `[macros]`[macros] defined in
`[std::fmt]`[fmt]. Some of which include:

- `format!`: write formatted text to `[String]`[string]
- `print!`: same as `format!` but the text is printed to the console (io::stdout)
- `println!`: same as `print!` but a newline is appended
- `eprint!`: same as `format!` but the text is printed to the standard
  error(io::stderr).
- `eprintln!`: same as `eprint!` but a newline is appended

All parse text in the same fashion. As a plus, Rust checks format correctness at
compile time.
