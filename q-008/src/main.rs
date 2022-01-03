// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
    }
    let mut answer = 0;
    for i in 1..=n {
        for j in 1..=n {
            if i + j <= s {
                answer += 1
            }
        }
    }
    println!("{}", answer);
}
