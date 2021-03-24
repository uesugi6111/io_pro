/// https://qiita.com/tatsuya6502/items/cd448486f7ef7b5b8c7e#%E6%80%A7%E8%83%BD%E6%B8%AC%E5%AE%9A%E7%94%A8%E3%83%87%E3%83%BC%E3%82%BF%E3%81%AE%E4%BD%9C%E6%88%90
use std::io::{BufWriter, Write};

fn main() {
    // 生成する入力データの行数（10^7）
    let count = 10usize.pow(7);

    // 標準出力をロックし、BufWriterで包んでバッファリングする。
    let stdout = std::io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    writeln!(out, "{}", count).expect("failed to write");
    for i in 0..count {
        writeln!(out, "{} -{}e3 {}", i, i, i).expect("failed to write");
    }

    // エラーを取りこぼさないよう明示的にflushする。
    // 参考： Rustといえどリソースの解放は注意
    //       http://keens.github.io/blog/2016/01/08/rusttoiedoriso_sunokaihouhachuui/
    out.flush().expect("failed to flush");
}
