//! ## Smallest multiple
//!
//! 2520 是最小的能被 1 到 10 的所有整数整除的数。求最小的能被 1 到 20
//! 的所有整数整除的数。

fn is_blue(n: u64) -> bool {
    for i in 1..=20 {
        if n % i != 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut n = 20;

    for _ in 0.. {
        if is_blue(n) {
            break;
        }
        n += 20;
    }

    dbg!(n);
}
