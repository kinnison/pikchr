Pikchr - Diagram renderer
=========================

Taken from the [pikchr](https://pikchr.org/home/doc/trunk/homepage.md) homepage:

> Pikchr (pronounced like "picture") is a [PIC][1]-like markup
> language for diagrams in technical documentation.  Pikchr is
> designed to be embedded in [fenced code blocks][2] of
> Markdown (or in similar mechanisms in other markup languages)
> to provide a convenient means of showing diagrams.
> 
> [1]: https://en.wikipedia.org/wiki/Pic_language
> [2]: https://spec.commonmark.org/0.29/#fenced-code-blocks

This crate wrappers the `pikchr.c` version downloaded from that website
on the 8th May 2021.

You can use it as follows:

```rust
use pikchr::{Pikchr, PikchrFlags};

let piccy = Pikchr::render(
    diagram_str,
    None,
    PikchrFlags::default()).unwrap();

println!("{}", piccy);
```

There is a little helper program that reads a Pikchr file named on the
command line and renders it as SVG to the standard output:

~~~sh
cargo run -p pikchr-cli -q foo.pikchr > foo.svg
~~~

You can install it with `cargo install pikchr-cli` (`cargo install
--path=pikchr-cli` from the source tree).
