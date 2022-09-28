use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a:[usize; n],
    }

    println!("{}", solve(n, x, a));
}

fn solve(n: usize, x: usize, a: Vec<usize>) -> &'static str {
    for v in a.iter() {
        if *v == x {
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
        assert_eq!(solve(5, 40, vec![10, 20, 30, 40, 50]), "Yes");
    }
}
