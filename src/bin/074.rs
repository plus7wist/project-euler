//! ## Digit factorial chains
//!
//! The number 145 is well known for the property that the sum of the factorial
//! of its digits is equal to 145:
//!
//! ```text
//! 1! + 4! + 5! = 1 + 24 + 120 = 145
//! ```
//!
//! Perhaps less well known is 169, in that it produces the longest chain of
//! numbers that link back to 169; it turns out that there are only three such
//! loops that exist:
//!
//! ```text
//! 169 → 363601 → 1454 → 169
//! 871 → 45361 → 871
//! 872 → 45362 → 872
//! ```
//!
//! It is not difficult to prove that EVERY starting number will eventually get
//! stuck in a loop. For example,
//!
//! ```text
//! 69 → 363600 → 1454 → 169 → 363601 (→ 1454)
//! 78 → 45360 → 871 → 45361 (→ 871)
//! 540 → 145 (→ 145)
//! ```
//!
//! Starting with 69 produces a chain of five non-repeating terms, but the
//! longest non-repeating chain with a starting number below one million is sixty
//! terms.
//!
//! How many chains, with a starting number below one million, contain exactly
//! sixty non-repeating terms?

use std::collections::VecDeque;

fn factorial_sum(mut n: u64, fmap: &[u64]) -> u64 {
	let mut sum = 0;
	while n > 0 {
		let d = (n % 10) as usize;
		sum += fmap[d];
		n /= 10;
	}
	sum
}

fn factorial_map() -> Vec<u64> {
	let mut fact_map = vec![];
	let mut fact = 1;
	for i in 1..=10 {
		fact_map.push(fact);
		fact *= i;
	}
	fact_map
}

fn main() {
	let fmap = factorial_map();
	let mut dist = vec![u64::MAX; 1_000_000];
	let mut edge = vec![vec![]; 1_000_000];

	for i in 0..edge.len() {
		let j = factorial_sum(i as u64, &fmap) as usize;

		if j >= edge.len() {
			continue;
		}
		edge[j].push(i);
	}

	let starts = vec![
		(1, 1),
		(2, 1),
		(145, 1),
		(871, 2),
		(45361, 2),
		(872, 2),
		(45362, 1),
		(169, 3),
		(363601, 3),
		(1454, 3),
	];

	let mut queue = VecDeque::new();

	for (n, v) in starts {
		queue.push_back(n);
		dist[n] = v;
	}

	while let Some(stand) = queue.pop_front() {
		let dist_stand = dist[stand];

		for &goto in &edge[stand] {
			if dist[goto] == u64::MAX {
				dist[goto] = dist_stand + 1;
				queue.push_back(goto);
			}
		}
	}

	dbg!(dist[69]);
	dbg!(dist[78]);
	dbg!(dist[540]);
	dbg!(dist.into_iter().filter(|&d| d == 60).count());
}
