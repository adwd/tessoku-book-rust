use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    println!("{}", solve(a, b));
}

fn solve(a: usize, b: usize) -> &'static str {
    for v in a..=b {
        if 100 % v == 0 {
            return "Yes";
        }
    }
    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(5, 20), "Yes");
        assert_eq!(solve(6, 9), "No");
        assert_eq!(solve(9, 10), "Yes");
    }
}
