use proconio::input;

fn main() {
    input! {
        n: String,
    }

    println!("{}", solve(n));
}

fn solve(n: String) -> usize {
    let mut sum = 0;
    for (index, c) in n.char_indices() {
        let k = n.len() - index - 1;
        if c == '1' {
            sum += 1 << k;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve("1101".to_string()), 13);
    }
}
