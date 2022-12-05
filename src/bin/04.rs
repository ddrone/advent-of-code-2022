#[derive(Debug)]
struct Interval {
    begin: u64,
    end: u64
}

impl Interval {
    fn from_str(s: &str) -> Self {
        let pair: Vec<&str> = s.split("-").collect();
        Interval { begin: pair[0].parse().unwrap(), end: pair[1].parse().unwrap() }
    }

    fn fully_contains(&self, other: &Self) -> bool {
        other.begin >= self.begin && other.end <= self.end
    }

    fn contains_end(&self, other: &Self) -> bool {
        (self.begin <= other.begin && other.begin <= self.end) ||
        (self.begin <= other.end && other.end <= self.end)
    }
}

fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("data/04.txt")?;
    let mut count: u64 = 0;
    for line in file.lines() {
        let pair: Vec<&str> = line.split(",").collect();
        let i1 = Interval::from_str(pair[0]);
        let i2 = Interval::from_str(pair[1]);

        if i1.fully_contains(&i2) || i2.fully_contains(&i1) || i1.contains_end(&i2) || i2.contains_end(&i1) {
            count += 1;
        }
    }
    println!("{}", count);
    Ok(())
}