buildmodes
==========

This repo demonstrates statically linking and calling Go code from
other languages.  It requires Go 1.5 or higher, which will introduce
additional [execution modes](https://docs.google.com/document/d/1nr-TQHw_er6GOQRsF6T43GGhFDelrAP0NqSS_00RgZQ/edit?pli=1)
to allow programs written in other languages to statically or
dynamically link to and call Go code using the system C ABI.
Rust was chosen for this demonstration because of its excellent
package manager Cargo and how simple it is to integrate seamlessly
with other build systems, but linking to other languages should be
relatively straightforward as well.

Demo
----

To build and run the entire project, simply run:

```bash
cargo run
```

The printed number (20) is calculated and returned by Go as the
multiplication of two numbers (4 and 5) passed by Rust.

Explanation
-----------

When Cargo builds the project, it compiles and executes `build.rs`,
which calls the `go` tool to build a C archive and writes it to a
Cargo build directory.

The `go build` call must be used to build a Go main package.  All cgo
exported functions (annotated with `//export FuncName` directives) are
exported by the archive.

See the [Cargo documentation](http://doc.crates.io/build-script.html)
for more details about build scripts.

If integrating with non-Cargo projects, the archive can be built
manually by running something like:

```bash
go build -buildmode=c-archive -o libbuildmode.a src/go/main.go
```

The Go source file (`src/go/main.go`) is a `main` package with a
single exported function:

```Go
//export Multiply
func Multiply(a, b int) int {
	return a * b
}
```

When using any of the C buildmodes, `cgo` creates the C function:

```C
int Multiply(int, int);
```

This is the function that Rust calls.  See the [cgo documentation](http://golang.org/cmd/cgo/)
for more details about how `cgo` creates C interfaces for Go code.

Since Rust does not understand C header files, external functions must
be declared manually in an `extern` block.  This is done in
`src/main.rs` in the `go` module:

```Rust
#[link(name = "buildmodes")]
extern {
    fn Multiply(a: c_int, b: c_int) -> c_int;
}
```

Since `rustc` may not make any assumptions about the safety of calling
non-Rust code, this function is automatically marked unsafe to call.
A safe Rust wrapper is created in the `go` module:

```Rust
pub fn multiply(a: c_int, b: c_int) -> c_int {
    unsafe { Multiply(a, b) }
}
```

It is then called by `main`, and the result is printed to `stdout`:

```Rust
fn main() {
    println!("{}", go::multiply(4, 5));
}
```
