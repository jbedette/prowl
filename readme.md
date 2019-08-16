# Prowl: PROcedural WorLd generator

Prowl is an experiment with procedural generation written in Rust by Matt
O'Tousa and John Bedette for Bart Massey's Summer 2019 Rust course at Portland
State University.

## `libtcod` Dependencies

### Building on Linux

Run the equivalent of:

```sh
$ sudo apt-get install gcc g++ make libsdl2-dev
$ cd yourgame
$ cargo build --release
$ cargo run --release
```

on your distro.

#### Building a dynamic library

By default, `tcod-rs` will build the library statically on Linux as including
the code into the executable is usually more convenient. To build a dynamic
library specify the `dynlib` feature for `tcod-sys` in `Cargo.toml`

```
[dependencies.tcod-sys]
version = "*"
features = ["dynlib"]
```

### Building on Windows (with MSVC)

Make sure you have Visual Studio 2013 or later **with the C++ tools
option** installed. You also need the "MSVC ABI" version of the Rust
compiler (as opposed to the "GNU ABI" one).

Then, set up the compilation environment, make sure Rust is in your
`PATH` and run Cargo:

```
C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat amd64
set PATH=%PATH%;C:\Program Files (x86)\Rust\bin
cd yourgame
cargo build --release
cargo run --release
```


### Building on Windows (with MinGW)

You have to [download and install MinGW](http://www.mingw.org/). Then,
add Rust's and MinGW's bin directories to your path and compile your
game:

```
set PATH=%PATH%;C:\Program Files (x86)\Rust\bin;C:\MinGW\bin
cd yourgame
cargo build --release
cargo run --release
```


### Building on Mac OS X

1. [Install Homebrew](http://brew.sh/)
2. Run:

```sh
$ brew install pkg-config sdl2
$ cd yourgame
$ cargo build --release
$ cargo run --release
```

This is based on the instructions from [Jared McFarland's roguelike tutorial](http://jaredonline.svbtle.com/roguelike-tutorial-in-rust-part-1).

## Design

Prowl heavily utilizes the [specs](https://github.com/slide-rs/specs) crate for
its logic and infrastructure. It also utilizes specs' shred dispatcher system,
which runs specs' systems concurrently when possible.

It uses the C library [libtcod](https://github.com/libtcod/libtcod) through the
Rust wrapper [tcod-rs](https://github.com/tomassedovic/tcod-rs). It only uses
this library for input and rendering, and uses Rust solutions for random number
and noise generation instead of the `tcod` built-ins because we would like to
migrate everything to a pure Rust renderer at some point.

## Notes

Entity-Component-System hierarchy was a new way of thinking for us and more time
than we expected was devoted to just figuring out reasonable ways to make
different things interact with each other.

That being said, once we laid a foundation, adding new systems required only
thinking about the components that system touches so it was relatively painless
to add new features -- so long as those features built on foundation we had
already laid.

We also had tried several different 'renderers' before we settled on `tcod`. First
we tried [Amethyst](https://amethyst.rs/) for an initial 3D test
(which would have been very cool but seemed like too much work). After that,
we determined we wanted to use a rogue-like-style terminal display to keep things
simple. So we tried out [Termion](https://github.com/redox-os/termion) but it
was slow and flickery. This may very well be to misusing its API somehow, but
when we discovered [tcod-rs](https://github.com/tomassedovic/tcod-rs) and
realized how ubiquitous `libtcod` was amongst rogue-likes it seemed like an
easy candidate. It's not a real terminal emulator but a graphical application
front-end that feels like a terminal, but it ran fast and was easy to use so
it was an easy choice.

If we had more time we likely would have given
(Cursive)[https://docs.rs/cursive/0.13.0/cursive/] a shot. It supports different
terminal back-ends and may very well use Termion more correctly than we did.
