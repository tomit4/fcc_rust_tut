## Free Coded Camp's Introduction To Rust Tutorial

This repo holds files related to [Free Code Camp's Intro To Rust Tutorial](https://www.youtube.com/watch?v=BpPEoZW5IiY).

Ignored from git is the output of the binaries, to avoid bloating of this repo.
When following along in the tutorial, create a `./bin` directory in this project
directory. When compiling using `rustc`, make sure to output somewhere in the
`./bin` directory like so:

```sh
rustc main.rs -o ../bin/tutorial_subect/main
```

You can then simply call the binary like so:

```sh
../bin/tutorial_subect/main
```
