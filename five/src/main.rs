use std::collections::VecDeque;

fn parse_stack_line(i: &str) -> Vec<Option<String>> {
    let mut res = Vec::new();
    let chars = i.chars().collect::<Vec<_>>();
    for chunk in chars.chunks(4) {
        if chunk[0] == ' ' {
            res.push(None);
        } else {
            res.push(Some(chunk[1].to_string()));
        }
    }

    res
}

fn parse_operations(i: &str) -> (usize, usize, usize) {
    let split = i.split(' ').collect::<Vec<_>>();
    (
        split[1].parse::<usize>().expect("number of cargo to move not parsable"),
        split[3].parse::<usize>().expect("from stack not parsable"),
        split[5].parse::<usize>().expect("to stack not parsable")
    )
}

fn main() {
    let lines = std::io::stdin().lines().collect::<Result<Vec<_>, _>>().expect("couldn't get lines from stdin");

    let stack_lines = lines.iter().take_while(|l| !l.starts_with(" 1")).collect::<Vec<_>>();
    let stacks =
        stack_lines
        .iter()
        .map(|l| parse_stack_line(&l))
        .rev()
        .fold(
            Vec::<VecDeque<String>>::new(),
            |mut stacks, stack_line| {
                for (idx, stack_item) in stack_line.into_iter().enumerate() {
                    if let Some(stack_item) = stack_item {
                        if let Some(stack) = stacks.get_mut(idx) {
                            stack.push_front(stack_item);
                        } else {
                            let mut stack = VecDeque::new();
                            stack.push_front(stack_item);
                            stacks.insert(idx, stack);
                        }
                    }
                }

                stacks
            });


    let ops_lines = lines[(stack_lines.len() + 2)..]
        .iter()
        .map(|i| parse_operations(&i))
        .collect::<Vec<_>>();

    let mut stacks1 = stacks.clone();
    for (num, from, to) in ops_lines.clone() {
        for _ in 0..num {
            let cargo = stacks1.get_mut(from - 1).expect("Should have an to stack").pop_front().expect("should contain cargo");
            stacks1[to - 1].push_front(cargo);
        }
    }

    let mut res = Vec::new();
    for mut stack in stacks1 {
        if let Some(item) = stack.pop_front() {
            res.push(item);
        }
    }

    println!("{}", res.into_iter().collect::<String>());

    let mut stacks2 = stacks.clone();
    for (num, from, to) in ops_lines {
        let mut crane = Vec::new();
        for _ in 0..num {
            let cargo = stacks2.get_mut(from - 1).expect("Should have an to stack").pop_front().expect("should contain cargo");
            crane.push(cargo);
        }

        for cargo in crane.into_iter().rev() {
            stacks2[to - 1].push_front(cargo);
        }
    }

    let mut res = Vec::new();
    for mut stack in stacks2 {
        if let Some(item) = stack.pop_front() {
            res.push(item);
        }
    }

    println!("{}", res.into_iter().collect::<String>());
}
