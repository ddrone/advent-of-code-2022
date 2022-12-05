use std::collections::HashSet;
pub fn score(c: char) -> u8 {
    if c >= 'a' && c <= 'z' {
        c as u8 - 'a' as u8 + 1
    }
    else {
        c as u8 - 'A' as u8 + 27
    }
}

fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("data/03.txt")?;
    let mut sum: u64 = 0;
    for line in file.lines() {
        let v: Vec<char> = line.chars().collect();
        let n = line.len() / 2;
        let mut first: HashSet<char> = HashSet::new();
        for i in 0..n {
            first.insert(v[i]);
        }
        for i in n..line.len() {
            let c = v[i];
            if first.contains(&c) {
                sum += score(c) as u64;
                break;
            }
        }
    }
    println!("{}", sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::score;

    #[test]
    fn score_correct() {
        assert_eq!(score('b'), 2);
        assert_eq!(score('C'), 29);
    }
}