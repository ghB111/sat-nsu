# sat-nsu


Install rust and cargo: https://rustup.rs/ 

Build executable:

```
cargo build
./target/debug/sat-nsu ./data/formulas/2sat_test0.cnf
```

Build and run a specific formula:

```sh
cargo run ./data/formulas/2sat_test1.cnf
```

Also you can specify method:

```
USAGE:
    sat-nsu [OPTIONS] <input>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --method <method>    Method to use when solving [default: recursion]  [possible values: recursion, no-recursion]
```

Unit tests:

```sh
cargo test
```

