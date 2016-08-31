# collatz

`collatz` is a project I started to write some math-oriented Rust as
well as investigating the [Collatz Conjecture] \(which I first heard
about in a [video on Numberphile]).

## Operation

### Version 1

In the first version, the program tests numbers for convergence,
starting at one and counting upward in batches of 100,000. To make the
calculation slightly more efficient, it stores a cache of previously
checked numbers. This cache enables shortcutting the calculation of the
sequence when it leads to a converging number.

The cache is a [BTreeSet] for now. The speed is acceptable for numbers
under three million, but after that point, the calculations take
considerably more time. The performance hit is likely due to the number
of entries in the cache set. I'd like to try my hand at a set that's
based on ranges stored in a  binary tree to get back some performance.

## Running

With Rust and Cargo installed ([rustup]), running is as easy as:

```
$ git clone https://github.com/ryanq/collatz
$ cd collatz
```

or:

```
$ hub clone ryanq/collatz
$ cd collatz
```

then (using the stable channel):

```
$ cargo build --release
$ ./target/release/collatz
```

[Collatz Conjecture]: https://en.wikipedia.org/wiki/Collatz_conjecture
[video on Numberphile]: https://www.youtube.com/watch?v=5mFpVDpKX70
[BTreeSet]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html
[rustup]: https://www.rustup.rs