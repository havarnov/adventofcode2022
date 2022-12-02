use std::io;

enum Hand {
    Rock,
    Paper,
    Scissor,
}

fn from_round(input: &str) -> (Hand, Hand) {
    let mut iter = input.split(' ');
    (to_hand(iter.next().unwrap()), to_hand(iter.next().unwrap()))
}

fn to_hand(input: &str) -> Hand {
    match input {
        "A" | "X" => Hand::Rock,
        "B" | "Y" => Hand::Paper,
        "C" | "Z" => Hand::Scissor,
        _ => panic!("Couldn't create hand."),
    }
}

fn score((fst, snd): (Hand, Hand)) -> usize {
    use Hand::*;
    match (fst, snd) {
        (Rock, Rock) => 1 + 3,
        (Rock, Paper) => 2 + 6,
        (Rock, Scissor) => 3 + 0,

        (Paper, Rock) => 1 + 0,
        (Paper, Paper) => 2 + 3,
        (Paper, Scissor) => 3 + 6,

        (Scissor, Rock) => 1 + 6,
        (Scissor, Paper) => 2 + 0,
        (Scissor, Scissor) => 3 + 3,
    }
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

fn to_outcome(input: &str) -> Outcome {
    match input {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("Couldn't create outcome."),
    }
}

fn from_round2(input: &str) -> (Hand, Outcome) {
    let mut iter = input.split(' ');
    (to_hand(iter.next().unwrap()), to_outcome(iter.next().unwrap()))
}

fn find_hand(input: (&Hand, &Outcome)) -> Hand {
    use Hand::*;
    use Outcome::*;
    match input {
        (Rock, Lose) => Scissor,
        (Rock, Draw) => Rock,
        (Rock, Win) => Paper,

        (Paper, Lose) => Rock,
        (Paper, Draw) => Paper,
        (Paper, Win) => Scissor,

        (Scissor, Lose) => Paper,
        (Scissor, Draw) => Scissor,
        (Scissor, Win) => Rock,
    }
}

fn main() {
    let lines = io::stdin().lines().collect::<Result<Vec<String>, _>>().unwrap();
    let res =
        lines
        .iter()
        .map(|i| from_round(&i))
        .fold(
            0,
            |s, n| s + score(n));

    println!("{}", res);

    let res =
        lines
        .iter()
        .map(|i| from_round2(&i))
        .map(|(hand, outcome)| {
          let snd_hand = find_hand((&hand, &outcome));
          (hand, snd_hand)
        })
        .fold(
            0,
            |s, n| s + score(n));

    println!("{}", res);
}
