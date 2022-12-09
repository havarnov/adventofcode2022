fn main() {
    let lines = std::io::stdin().lines().collect::<Result<Vec<_>,_>>().expect("couldn't read input as lines");

    let ops = lines
        .iter()
        .map(|l| {
            let mut split = l.split(' ');
            (
                split.next().expect("get op").to_string(),
                split.next().expect("get num").parse::<usize>().expect("parse num as usize"),
            )
        })
        .collect::<Vec<_>>();

    let mut visited = std::collections::HashSet::new();
    visited.insert((0isize, 0isize));

    let mut head = (0isize, 0isize);
    let mut tail = (0isize, 0isize);

    for (op, num) in ops.iter() {
        for _ in 0..*num {
            match op.as_str() {
                "U" => head = (head.0, head.1 + 1),
                "D" => head = (head.0, head.1 - 1),
                "L" => head = (head.0 - 1, head.1),
                "R" => head = (head.0 + 1, head.1),
                _ => unreachable!("shouldn't be possible"),
            }

            if tail.0 >= head.0 - 1
               && tail.0 <= head.0 + 1
               && tail.1 >= head.1 - 1
               && tail.1 <= head.1 + 1 {
                continue;
            }

            
            match op.as_str() {
                "U" => 
                    tail = (head.0, head.1 - 1),
                "D" =>
                    tail = (head.0, head.1 + 1),
                "L" => 
                    tail = (head.0 + 1, head.1),
                "R" => 
                    tail = (head.0 - 1, head.1),
                _ =>
                    unreachable!("shouldn't be possible"),
            }

            visited.insert(tail);
        }
    }

    println!("{}", visited.len());

    let mut visited = std::collections::HashSet::new();
    visited.insert((0isize, 0isize));

    let mut rope = vec![(0isize, 0isize); 10];

    for (op, num) in ops.iter() {

        for _ in 0..*num {
            let head = rope[0];
            let new_head = match op.as_str() {
                "U" => (head.0, head.1 + 1),
                "D" => (head.0, head.1 - 1),
                "L" => (head.0 - 1, head.1),
                "R" => (head.0 + 1, head.1),
                _ => unreachable!("shouldn't be possible"),
            };

            _ = std::mem::replace(&mut rope[0], new_head);

            for i in 1..10 {
                let head = rope[i - 1];
                let tail = rope[i];

                if tail.0 >= head.0 - 1
                   && tail.0 <= head.0 + 1
                   && tail.1 >= head.1 - 1
                   && tail.1 <= head.1 + 1 {
                    continue;
                }

                let new_x =
                    if tail.0 > head.0 {
                        tail.0 - 1
                    } else if tail.0 < head.0 {
                        tail.0 + 1
                    } else {
                        tail.0
                    };

                let new_y =
                    if tail.1 > head.1 {
                        tail.1 - 1
                    } else if tail.1 < head.1 {
                        tail.1 + 1
                    } else {
                        tail.1
                    };

                let new_tail = (new_x, new_y);

                _ = std::mem::replace(&mut rope[i], new_tail);

                if i == 9 {
                    visited.insert(new_tail);
                }
            }
        }
    }

    println!("{}", visited.len());
}
