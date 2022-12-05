fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("data/01-a.txt")?;
    let mut curr_sum: u64 = 0;
    let mut result: u64 = 0;
    for line in file.lines() {
        if line.is_empty() {
            result = result.max(curr_sum);
            curr_sum = 0;
        }
        else {
            let x: u64 = line.parse().unwrap();
            curr_sum += x;
        }
    }
    println!("{}", result);
    Ok(())
}