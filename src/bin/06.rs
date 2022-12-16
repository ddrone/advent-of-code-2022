use std::collections::HashSet;


fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("data/06.txt")?;
    let chars: Vec<char> = file.chars().collect();

    for i in 0..chars.len() {
        let distinct: HashSet<char> = HashSet::from_iter(chars[i..i+4].iter().map(|x| *x));
        if distinct.len() == 4 {
            println!("{}", i + 4);
            break;
        }
    }

    Ok(())
}