## panic!

- The simplest form of <b>error handling</b> is to use the `panic!` macro

- `panic!` will print out an <b>error message</b>, <b>unwind</b> and
  <b>stack</b> and finally <b>exit</b> the program

- In `multithreaded` programs, the program will <b>exit</b> the thread in which
  the `panic!` occurs, not the entire program
