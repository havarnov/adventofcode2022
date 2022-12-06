use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    _ = std::io::stdin()
        .read_line(&mut input)
        .expect("couldn't read single line");

    let (_, res) = input
        .chars()
        .enumerate()
        .fold(
            ((None, None, None, None), None),
            |((a, b, c, d), r), (idx, next_c)| {
                if r.is_some() {
                    return ((a, b, c, d), r);
                }

                match (b, c, d, Some(next_c)) {
                    (Some(a), Some(b), Some(c), Some(d)) if a != b && a != c && a != d && b != c && b != d && c != d =>
                        ((Some(a), Some(b), Some(c), Some(d)), Some(idx)),
                    _ => ((b, c, d, Some(next_c)), None),
                }
            });

    println!("{:?}", res.map(|r| r + 1));

    let (_, res) = input
        .chars()
        .enumerate()
        .fold(
            (Vec::new(), None),
            |(mut s, r), (idx, next_c)| {
                if r.is_some() {
                    return (s, r);
                }

                s.insert(0, next_c);
                if s.len() < 14 {
                    return (s, r);
                }

                let set: HashSet<_> = s.iter().take(14).collect();

                if set.len() == 14 {
                    return (s, Some(idx));
                }

                (s, r)
            });

    println!("{:?}", res.map(|r| r + 1));
}
