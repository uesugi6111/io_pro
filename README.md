# io_pro

競技プログラミング用の標準入力

## 計測方法
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
time ./target/release/proconio < ./test.in
```

## 結果
5回の平均  
単位はms
|        |  1e5  |    1e7   |
|  ----  | ----  |   ----   |
|proconio|  40   |  2315.6  |
| io_pro |  66   |  5492.4  |
