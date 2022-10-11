//! ## 10001-st prime
//!
//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see
//! that the 6-th prime is 13.
//!
//! What is the 10001-st prime number?

fn main() {
	let mut listing = vec![2];
	let maxn = 10001;

	for i in 3.. {
		let mut is_prime = true;
		for p in &listing {
			if i % p == 0 {
				is_prime = false;
				break;
			}
		}

		if is_prime {
			listing.push(i);

			if listing.len() >= maxn {
				break;
			}
		}
	}

	dbg!(&listing[..10]);
	dbg!(listing[maxn - 1]);
}
