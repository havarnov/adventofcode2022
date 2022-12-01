use std::io;

fn main() -> io::Result<()> {
    let mut res = io::stdin().lines()
        .collect::<Result<Vec<String>, _>>()?
        .iter()
        .fold(
            (Vec::new(), None),
            |(mut list, current), next| {
                match (current, next.as_str()) {
                    (Some(current), "") => {
                        list.push(current);
                        (list, None)
                    },
                    (Some(current), number_as_str) => {
                        let number = number_as_str.parse::<usize>().unwrap();
                        (list, Some(current + number))
                    },
                    (None, number_as_str) => {
                        let number = number_as_str.parse::<usize>().unwrap();
                        (list, Some(number))
                    }
                }
            })
        .0;

    res.sort();
    res.reverse();

    println!("{}", res[0]);
    println!("{}", res.iter().take(3).sum::<usize>());


    Ok(())
}
