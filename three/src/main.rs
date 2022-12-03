use std::error::Error as StdError;
use std::io;

type Res<T> = Result<T, Box<dyn StdError>>;

fn find_dup(i: &[char]) -> Option<char> {
    let len = i.len() / 2;
    let fst = &i[..len];
    let snd = &i[len..];
    for c in snd {
        if fst.contains(&c)
        {
            return Some(c.clone());
        }
    }

    None
}

fn find_dup2(i: &[Vec<char>]) -> Option<char> {
    let fst = &i[0];
    let snd = &i[1];
    let thr = &i[2];
    
    let mut seen = Vec::new();

    for c in snd {
        if fst.contains(&c) {
            seen.push(c.clone());
        }
    }

    for c in seen {
        if thr.contains(&c) {
            return Some(c.clone());
        }
    }


    None
}

fn find_value(i: &char) -> usize {
    if i.is_ascii_uppercase() {
        *i as usize - 65 + 27
    } else {
        *i as usize - 97 + 1
    }
}

fn main() -> Res<()> {
    let lines = io::stdin()
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

    let res1 =
        lines
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .map(|s| find_dup(&s).ok_or("dup not found"))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .map(find_value)
        .fold(0, |s, n| s + n);

    println!("{}", res1);

    let res2 =
        lines
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .as_slice()
        .chunks(3)
        .map(find_dup2)
        .map(|o| o.ok_or("dup not found"))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .map(find_value)
        .fold(0, |s, n| s + n);

    println!("{}", res2);


    Ok(())
}
