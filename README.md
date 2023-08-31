<p align=center>
  <img src="./.statics/logo.png" width="320" alt="QUX Logo" /></a>
</p>

<p align=center  style="background-color: #1a1b1c; color: #c3a268; " > Browser Engine </p>

Instructions
------------

1. [Install Rust](http://www.rust-lang.org/install.html)

2. Clone the QUX source code from https://github.com/mpf0007/QUX

3. Run `cargo build` to build QUX, and `cargo run` to run it.

To build and run with optimizations enabled, use `cargo build --release` and
`cargo run --release`.

By default, QUX will load test.html and test.css from the `examples`
directory.  You can use the `--html` and `--css` arguments to the QUX
executable to change the input files:

    ./target/debug/QUX --html examples/test.html --css examples/test.css

The rendered page will be saved to a file named `output.pdf`.  To change the output filename, use the `-o` option.  To switch to PNG output, use add
`--format png`.