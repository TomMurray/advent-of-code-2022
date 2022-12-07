use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn points(&self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Res {
    Win,
    Draw,
    Loss,
}

impl Res {
    fn points(&self) -> i32 {
        match self {
            Res::Win => 6,
            Res::Draw => 3,
            Res::Loss => 0,
        }
    }
}

fn get_their_choice(token: &str) -> Choice {
    match token {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissors,
        other => panic!("Unexpected choice (theirs) {}", other),
    }
}

// Note: This was used for part 1 of the challenge
#[allow(dead_code)]
fn get_my_choice(token: &str) -> Choice {
    match token {
        "X" => Choice::Rock,
        "Y" => Choice::Paper,
        "Z" => Choice::Scissors,
        other => panic!("Unexpected choice (mine) {}", other),
    }
}

fn get_desired_result(token: &str) -> Res {
    match token {
        "X" => Res::Loss,
        "Y" => Res::Draw,
        "Z" => Res::Win,
        other => panic!("Unexpected desired result {}", other),
    }
}

// Note: This was used for part 1 of the challenge
#[allow(dead_code)]
fn get_result(theirs: Choice, mine: Choice) -> Res {
    match mine {
        Choice::Rock => match theirs {
            Choice::Rock => Res::Draw,
            Choice::Paper => Res::Loss,
            Choice::Scissors => Res::Win,
        },
        Choice::Paper => match theirs {
            Choice::Rock => Res::Win,
            Choice::Paper => Res::Draw,
            Choice::Scissors => Res::Loss,
        },
        Choice::Scissors => match theirs {
            Choice::Rock => Res::Loss,
            Choice::Paper => Res::Win,
            Choice::Scissors => Res::Draw,
        },
    }
}

fn get_my_choice_for_result(theirs: Choice, res: Res) -> Choice {
    match theirs {
        Choice::Rock => match res {
            Res::Win => Choice::Paper,
            Res::Draw => Choice::Rock,
            Res::Loss => Choice::Scissors,
        },
        Choice::Paper => match res {
            Res::Win => Choice::Scissors,
            Res::Draw => Choice::Paper,
            Res::Loss => Choice::Rock,
        },
        Choice::Scissors => match res {
            Res::Win => Choice::Rock,
            Res::Draw => Choice::Scissors,
            Res::Loss => Choice::Paper,
        },
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input_file = File::open(path)?;
    let lines = BufReader::new(input_file).lines();

    let mut total: i32 = 0;
    for line in lines {
        if let Ok(entry) = line {
            let theirs = get_their_choice(&entry[0..1]);
            let res = get_desired_result(&entry[2..3]);

            let mine = get_my_choice_for_result(theirs, res);

            total += mine.points() + res.points();
        }
    }

    println!("Total points over all rounds was {}", total);

    Ok(())
}
