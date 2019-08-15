# Hangman

Copyright (c) 2019 Bader Alshaya

This is a mockup program of the infamous Hangman word game. The [words list](https://www.mit.edu/~ecprice/wordlist.10000) was pulled from the given website and was modified to only include non-compound English words that are less than 10-characters long. Finally, the list was stored as a text file inside the .txt file `data/words.txt` to use it as a simple data base for this program.

Other public sources used in this program will be listed below:
- https://github.com/mackwic/colored


## Build and Run

Build this program and library with `cargo build`. You can
run the program with `cargo run`.

You will need to pass a
`--` before a program flag.

To build or run an optimized version, use `cargo --release`.

Run `cargo test` to test the current version.


## License

This program is licensed under the "MIT License". Please
see the file `LICENSE` in the source distribution of this
software for license terms.
