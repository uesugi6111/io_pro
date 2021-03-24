# io_pro

競技プログラミング用の標準入力

テストケース作成
```
cargo run --release --bin generator > test.in
```

ビルド
```
cargo build --release --bin io_pro
cargo build --release --bin proconio
```

計測
```
time ./target/release/io_pro < ./test.in
time ./target/release/proconio  < ./test.in
```