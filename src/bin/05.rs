fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("data/05.txt")?;
    let lines: Vec<&str> = file.lines().collect();
    let split_index = lines.iter().position(|&s| s == "").unwrap();
    let stacks_count = lines[0].len() / 4 + 1;
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _i in 0..stacks_count {
        stacks.push(Vec::new());
    }

    let stack_lines = split_index - 1;
    for line in lines[..stack_lines].iter().rev() {
        let chars: Vec<char> = line.chars().collect();
        for i in 0..stacks_count {
            let char_index = i * 4 + 1;
            if chars[char_index] != ' ' {
                stacks[i].push(chars[char_index]);
            }
        }
    }

    println!("{:?}", stacks);

    // Example line to parse:
    // move 2 from 4 to 6
    for line in &lines[split_index+1..] {
        let words: Vec<&str> = line.split_whitespace().collect();
        let count: i32 = words[1].parse().unwrap();
        let mut from: i32 = words[3].parse().unwrap();
        from -= 1;
        let mut to: i32 = words[5].parse().unwrap();
        to -= 1;


        let mut items_to_push = {
            let stack_from = &mut stacks[from as usize];
            Vec::from(&stack_from[((stack_from.len() as i32 - count) as usize)..])
        };
        stacks[to as usize].append(&mut items_to_push);
        let new_size = stacks[from as usize].len() as i32 - count;
        stacks[from as usize].truncate(new_size as usize);
    }

    println!("{:?}", stacks);

    for stack in &stacks {
        print!("{}", stack.last().unwrap());
    }
    println!("");

    Ok(())
}