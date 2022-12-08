fn main() {
    let lines = std::io::stdin().lines().collect::<Result<Vec<_>,_>>().expect("couldn't read lines");

    let width = lines[0].len();
    let height = lines.len();
    let grid =
        lines
        .iter()
        .fold(
            Vec::new(),
            |mut s, line| {
                for d in line.chars().map(|c| c.to_string().parse::<usize>().expect("couldn' parse char to digit")) {
                    s.push(d);
                }
                s
            });

    let mut visible = std::collections::HashSet::new();

    for i in 1..(height - 1) {
        let row_start = i*width;

        let mut max = 0;
        for j in 1..(width - 1) {
            max = std::cmp::max(max, grid[row_start + j - 1]);
            if grid[row_start + j] > max {
                visible.insert(row_start + j);
            }
        }

        let mut max = 0;
        for j in (1..(width - 1)).rev() {
            max = std::cmp::max(max, grid[row_start + j + 1]);
            if grid[row_start + j] > max {
                visible.insert(row_start + j);
            }
        }
    }

    for i in 1..(width - 1) {
        let mut max = 0;
        for j in 1..(height - 1) {
            max = std::cmp::max(max, grid[(j - 1)*width + i]);
            if grid[j*width + i] > max {
                visible.insert(j*width + i);
            }
        }

        let mut max = 0;
        for j in (1..(height - 1)).rev() {
            max = std::cmp::max(max, grid[(j + 1)*width + i]);
            if grid[j*width + i] > max {
                visible.insert(j*width + i);
            }
        }
    }

    println!("{:?}", visible.len() + width*2 + height*2 - 4);

    let mut max = 0;
    for i in 1..(height - 1) {
        for j in 1..(width - 1) {
            let pos = i*width + j;
            let value = grid[pos];


            let mut east = 0;
            for p in (pos + 1)..(pos + (width - j)) {
                if grid[p] < value {
                    east += 1;
                } else {
                    east += 1;
                    break;
                }
            }


            let mut west = 0;
            for p in ((i*width)..pos).rev() {
                if grid[p] < value {
                    west += 1;
                } else {
                    west += 1;
                    break;
                }
            }

            let mut north = 0;
            for p in ((pos - (i*width))..(pos - width + 1)).rev().step_by(width) {
                if grid[p] < value {
                    north += 1;
                } else {
                    north += 1;
                    break;
                }
            }

            let mut south = 0;
            for p in ((pos + width)..(pos + ((height - i - 1)*width) + 1)).step_by(width) {
                if grid[p] < value {
                    south += 1;
                } else {
                    south += 1;
                    break;
                }
            }

            max = std::cmp::max(max, north * west * south * east);
        }
    }

    println!("{:?}", max);
}
