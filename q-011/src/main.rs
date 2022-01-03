// -*- coding:utf-8-unix -*-

use proconio::input;

/// 素数を確認するコードは、素朴に実装すると計算量 O(N) となり遅い。
/// エラトステネスのふるいを使うことで計算量を O(N log log N) に改善できる。
fn main() {
    input! {
        n: usize,
    }

    // 配列の初期化
    let mut primes = vec![false; n + 1];
    for i in 2..=n {
        primes[i] = true;
    }

    // エラストテネスのふるい
    // 2 〜 ルートN までの整数を探索すれば答えが得られることが知られている
    let mut i = 2;
    while i * i <= n {
        if primes[i] {
            // i の値は素数のため true のままとする
            // i の倍数にあたる値は素数ではないため false にする
            let mut x = 2 * i;
            while x <= n {
                primes[x] = false;
                x += i;
            }
        }
        i += 1;
    }

    // n 以下の素数を小さい順に出力
    let mut answers = Vec::new();
    for i in 2..=n {
        if primes[i] {
            answers.push(i.to_string());
        }
    }
    println!("{}", answers.join(" "));
}
