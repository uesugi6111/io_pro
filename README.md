# io_pro

Standard input macro for competitive programming

## How to measure
generate testcase
```
cargo run --release --bin generator > test.in
```

build
```
cargo build --release --bin io_pro
cargo build --release --bin proconio
```

measure
```
time ./target/release/io_pro < ./test.in
time ./target/release/proconio < ./test.in
```

## result
Average of 5 times

|        |  1e5  |    1e7   |
|  ----  | ----  |   ----   |
|proconio|  40ms   |  2315.6ms  |
| io_pro |  35.2ms   |  1839.4ms  |
