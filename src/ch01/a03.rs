use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
        q: [usize; n],
    }

    println!("{}", solve(n, k, p, q));
}

fn solve(n: usize, k: usize, p: Vec<usize>, q: Vec<usize>) -> &'static str {
    for pp in p.iter() {
        for qq in q.iter() {
            if *pp + *qq == k {
                return "Yes";
            }
        }
    }
    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(5, 100, vec![17, 57, 99], vec![10, 36, 53]), "No");
        assert_eq!(
            solve(5, 53, vec![10, 20, 30, 40, 50], vec![1, 2, 3, 4, 5]),
            "Yes"
        );
    }
}
