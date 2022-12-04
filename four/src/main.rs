
type AocResult<T> = Result<T, Box<dyn std::error::Error>>;

fn to_range(i: &str) -> (usize, usize) {
    let mut i = i.splitn(2, '-');
    let f = i.next().expect("missing start of range").parse::<usize>().expect("first to be parsable as usize");
    let s = i.next().expect("missing end of range").parse::<usize>().expect("snd to be parsable as usize");
    (f, s)
}

fn is_overlap(f: &(usize, usize), s: &(usize, usize)) -> bool {
    f.0 >= s.0 && f.1 <= s.1
}

fn is_partial_overlap(f: &(usize, usize), s: &(usize, usize)) -> bool {
    (f.0 <= s.0 && f.1 >= s.0) || (f.0 <= s.1 && f.1 >= s.1)
}

fn main() -> AocResult<()> {
    let lines = std::io::stdin().lines().collect::<Result<Vec<_>,_>>()?;

    let res1 =
        lines
        .iter()
        .map(|l| l.splitn(2, ',').collect::<Vec<_>>())
        .map(|i| (to_range(i[0]), to_range(i[1])))
        .filter(|(f, s)| is_overlap(f, s) || is_overlap(s, f))
        .count();

    println!("{}", res1);

    let res2 =
        lines
        .iter()
        .map(|l| l.splitn(2, ',').collect::<Vec<_>>())
        .map(|i| (to_range(i[0]), to_range(i[1])))
        .filter(|(f, s)| is_overlap(f, s) || is_overlap(s, f) || is_partial_overlap(f, s))
        .count();

    println!("{}", res2);

    Ok(())
}
