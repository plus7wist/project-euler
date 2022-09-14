//! # Even Fibonacci numbers
//!
//! 斐波那契序列中的数都是由前两项加总得出，假设第一与第二项为1与2，则前十项分别为：
//!
//! ```text
//! 1，2，3，5，8，13，21，34，55，89
//! ```
//!
//! 考虑不超过四百万的斐波那契数，计算其中偶数斐波那契数的和。

fn main() {
    let maxfib = 400_0000;

    println!(
        "{}",
        FibIter::new()
            .take_while(|f| *f <= maxfib)
            .filter(|f| f % 2 == 0)
            .sum::<u64>()
    );
}

struct FibIter {
    a: u64,
    b: u64,
    n: u64,
}

impl FibIter {
    fn new() -> Self {
        Self { a: 1, b: 2, n: 1 }
    }
}

impl Iterator for FibIter {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let v;

        if self.n <= 2 {
            v = self.n
        } else {
            v = self.a + self.b;
            self.a = self.b;
            self.b = v;
        }
        self.n += 1;

        Some(v)
    }
}
