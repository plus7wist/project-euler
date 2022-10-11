//! It turns out that 12 cm is the smallest length of wire that can be bent to form an integer
//! sided right angle triangle in exactly one way, but there are many more examples.
//!
//! 12 cm: (3,4,5)
//! 24 cm: (6,8,10)
//! 30 cm: (5,12,13)
//! 36 cm: (9,12,15)
//! 40 cm: (8,15,17)
//! 48 cm: (12,16,20)
//!
//! In contrast, some lengths of wire, like 20 cm, cannot be bent to form an integer sided right
//! angle triangle, and other lengths allow more than one solution to be found; for example, using
//! 120 cm it is possible to form exactly three different integer sided right angle triangles.
//!
//! 120 cm: (30,40,50), (20,48,52), (24,45,51)
//!
//! Given that L is the length of the wire, for how many values of L ≤ 1,500,000 can exactly one
//! integer sided right angle triangle be formed?

// 参考：https://en.wikipedia.org/wiki/Pythagorean_triple
//
// 当 0 < n < m，n 和 m 是整数：
//   a = m^2 - n^2
//   b = 2mn
//   c = m^2 + n^2
//
// 组成毕氏三元组。
//
// 满足下面约束条件 P1 的 (a, b, c) 是基本的。毕氏三元组是基本的，即不是其它毕氏三元组的倍数。
//
// 约束条件 P1：n 和 m 有而且只有一个是奇数，同时互素。

fn main() {
	const N: usize = 1_500_000 + 1;

	let mut count = vec![0u8; N];

	for n in 1u64.. {
		let nn = n * n;

		if nn >= count.len() as u64 {
			break;
		}

		for m in (n + 1).. {
			let mm = m * m;

			if mm >= N as u64 {
				break;
			}

			// a, b, c is primitive iff n and m is co-prime and one of them is even.
			if !((m + n) % 2 == 1 && num::integer::gcd(n, m) == 1) {
				continue;
			}

			let a = mm - nn;
			let b = 2 * m * n;
			let c = mm + nn;
			let sum = a + b + c;

			let mut sum1 = sum;

			while let Some(count_ref) = count.get_mut(sum1 as usize) {
				*count_ref += 1;
				sum1 += sum;
			}
		}
	}

	dbg!(&count[120]);
	dbg!(count
		.iter()
		.copied()
		.filter(|&x| x == 1)
		.fold(0, |sum, x| sum + x as u32));
}
