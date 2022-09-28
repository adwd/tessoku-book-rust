use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", solve(n));
}

fn solve(n: usize) -> String {
    let mut s = vec!['a'; 10];
    (0..=9).for_each(|x| {
        let wari = 1 << x;
        let ans = (n / wari) % 2;
        s[9 - x] = std::char::from_digit(ans as u32, 10).unwrap();
    });

    s.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(13), "0000001101");
    }
}
