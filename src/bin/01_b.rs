fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("data/01-a.txt")?;
    let mut curr_sum: i64 = 0;
    let mut sums: Vec<i64> = Vec::new();
    for line in file.lines() {
        if line.is_empty() {
            sums.push(curr_sum);
            curr_sum = 0;
        }
        else {
            let x: i64 = line.parse().unwrap();
            curr_sum += x;
        }
    }
    sums.sort_by_key(|x| -x);
    println!("{}", sums[0] + sums[1] + sums[2]);
    Ok(())
}