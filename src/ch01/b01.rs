use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    println!("{}", solve(a, b));
}

fn solve(a: usize, b: usize) -> usize {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(2, 4), 6);
    }
}
