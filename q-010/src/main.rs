// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut answer = 1;
    for i in 1..=n {
        answer *= i;
    }
    println!("{}", answer);
}
