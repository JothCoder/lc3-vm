# LC-3 VM

Yet another rusty implementation of a virtual machine emulating the
[Little Computer 3](https://en.wikipedia.org/wiki/Little_Computer_3) (LC-3).

This project is based on a great
[tutorial](https://justinmeiners.github.io/lc3-vm/index.html) by Justin Meiners.

## Usage

- To run the `2048.obj` example (by [Ryan Pendleton](https://github.com/rpendleton/lc3-2048)), use:

  ```sh
  cargo run --release -- assets/2048.obj
  ```

- To run the `rogue.obj` example (by [Justin Meiners](https://github.com/justinmeiners/lc3-rogue)), use:

  ```sh
  cargo run --release -- assets/rogue.obj
  ```

## Documentation

To generate and view the (internal) docs, use:

```sh
cargo doc --no-deps --document-private-items --open
```

## License

This project is licensed under the [MIT license](LICENSE).
