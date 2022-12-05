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
    let lines: Vec<&str> = file.lines().collect();
    for group in lines.chunks(3) {
        let mut map: HashSet<char> = HashSet::new();
        group[0].chars().for_each(|c| { map.insert(c); });

        let mut map2: HashSet<char> = HashSet::new();
        for c in group[1].chars() {
            if map.contains(&c) {
                map2.insert(c);
            }
        }

        for c in group[2].chars() {
            if map2.contains(&c) {
                // Badge found
                sum += score(c) as u64;
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