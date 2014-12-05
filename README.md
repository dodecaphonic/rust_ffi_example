# A Rust FFI example

A tiny example in wrapping a C library in Rust, with probably very bad code and wrong idioms. The idea was working out how to pass structs in -- case in point, something that can have nullable pointers, which would not work with declaring a `Box<T>`, as I understand it.

Running:

```
$ cargo run
```

It should print the following:

```
126.959649
600
```

