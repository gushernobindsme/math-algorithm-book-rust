// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }

    let mut answer = 0;
    for i in 1..=n {
        if i % x == 0 || i % y == 0 {
            answer += 1
        }
    }
    println!("{}", answer);
}
