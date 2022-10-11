//! ## Largest palindrome product
//!
//! 回文数就是从前往后读和从后往前读都一样的数。由两个2位数相乘得到的最大的回文数是
//! 9009 = 91 x 99。
//!
//! 求由两个3位数相乘得到的最大的回文数。

fn is_blue(n: u64) -> bool {
	let sn = format!("{}", n).into_bytes();

	let rs: Vec<_> = sn.iter().copied().rev().collect();

	sn == rs
}

fn main() {
	let mut max_blue = 0;

	for a in 100..=999 {
		for b in 100..=999 {
			if is_blue(a * b) {
				max_blue = max_blue.max(a * b);
			}
		}
	}

	dbg!(max_blue);
}
