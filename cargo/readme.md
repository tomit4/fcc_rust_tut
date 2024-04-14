## Cargo

- Cargo is the official <b>package manager</b> and <b>building tool</b> for
  Rust

- Cargo helps <b>automate tasks</b> such as creating new projects, building
  running, and testing code, as well as managing dependencies

- A <b>crate</b> is a compilation unit of Rust source code

- The site, <b>[crates.io](https://crates.io/)</b>, is a repository for Rust
  packages

### Crate

A crate can be of two types, a <b>Binary</b> or a <b>Library</b>.

<b>- Binary</b>

- Compiled into an <b>executable</b> binary
- Basically a "program"

<b>- Library</b>

- Compiled into a <b>library</b>
- <b>Reusable code</b> that can be shared across multiple projects

<b>- Crate root</b>

- Source file that is the <b>root module</b> of the crate
- In binaries: src/main.rs
- In libraries: src/lib.rs

### Modules

- Modules are a way of <b>organizing</b> code by <b>grouping</b> related items
  together

- Modules can be <b>imported</b> using <b>namespaces</b> avoiding naming
  collisions

- Modules also control the <b>privacy of its items</b> like functions, structs,
  enums, etc.

- When compiling, the Rust compiler starts from the <b>crate root</b>, then
  checks if modules are declared and looks for submodules

- <b>Submodules</b> could be <b>directly written inline</b> within <b>curly
  braces</b> (i.e. `{}`), in a <b>file</b> which has the <b>module name</b> ending in
  <b>`.rs`</b> or in a <b>directory</b> which has the <b>name of the module</b> and
  a <b>`mod.rs` file</b> inside it

**_Personal Notes:_**

Note the differences between a binary and library crate. One can simply create a
simple binary crate by invoking `cargo new` without any arguments other than the
project name like so:

```sh
cargo new hello-package
```

Within this directory, we can add library crates by adding a `lib.rs` file within the `src` directory:

```sh
cd hello_package &&\
touch src/lib.rs
```

There can <b>ONLY</b> be <b>ONE</b> library crate, but you can have as many
other binary crates you want, with different names, etc. You can do this by
creating a `bin` subdirectory within the `src` directory:

```sh
mkdir src/bin &&\
touch src/bin/main1.rs &&\
touch src/main2.rs
```

By invoking the `tree` command, we should have a directory structure that
resembles the following:

```sh
.
|__ Cargo.toml
|__ Cargo.lock
|__ src
|  |__ lib.rs
|  |__ main.rs
|  |__ bin
|     |__ main1.rs
|     |__ main2.rs
|__ tests # directory for integrated tests files
|   |__ some_integration_tests.rs
|__ benches # directory for benchmark files
|   |__ simple_bench.rs
|__ examples # directory for example files
|   |__ simple_example.rs
```
