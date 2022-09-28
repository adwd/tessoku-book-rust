use proconio::input;

fn main() {
    input! {
        n: usize,
        a:[usize; n],
    }

    println!("{}", solve(n, a));
}

fn solve(n: usize, a: Vec<usize>) -> &'static str {
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if a[i] + a[j] + a[k] == 1000 {
                    return "Yes";
                }
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
        assert_eq!(solve(5, vec![100, 250, 350, 400, 600]), "Yes");
    }
}
