use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    println!("{}", solve(n, k));
}

fn solve(n: usize, k: usize) -> usize {
    let mut sum = 0;
    for a in 1..=n {
        for b in 1..=n {
            let l = k - a - b;
            if 1 <= l && l <= n {
                sum += 1;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(3, 6), 7);
    }
}
