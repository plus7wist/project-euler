//! ## Largest prime factor
//!
//! 13195 的质因数是5、7、13、29。
//!
//! 求 600851475143 的最大质因数。

fn main() {
	let mut n = 600851475143_u64;
	let mut maxfact = 0;

	for i in 2.. {
		if n % i == 0 {
			maxfact = maxfact.max(i);
		}

		while n % i == 0 {
			n /= i;
		}

		if n / i <= i {
			break;
		}
	}

	if n > 1 {
		maxfact = maxfact.max(n);
	}

	dbg!(maxfact);
}
