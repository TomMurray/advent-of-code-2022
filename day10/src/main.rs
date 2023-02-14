use std::{env, error::Error, fs::File, io::{BufRead, BufReader}, str::FromStr, fmt};

enum Instruction {
    Noop,
    Addx(i32)
}

#[derive(Clone, Debug)]
struct InstructionParseError(String);

impl fmt::Display for InstructionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not parse instruction '{}'", self.0)
    }
}

impl Error for InstructionParseError{}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let name = split.next();
        if let Some(name) = name {
            match name {
                "noop" => Ok(Instruction::Noop),
                "addx" => {
                    let operand = split.next();
                    if let Some(operand) = operand {
                        let operand = operand.parse::<i32>();
                        match operand {
                            Ok(count) => {
                                Ok(Instruction::Addx(count))
                            },
                            Err(err) => {
                                Err(InstructionParseError(err.to_string()))
                            }
                        }
                    } else {
                        Err(InstructionParseError(String::from(s)))
                    }
                },
                _ => Err(InstructionParseError(format!("Unrecognised instruction '{}'", name)))
            }
        } else {
            Err(InstructionParseError(String::from(s)))
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let input = File::open(input)?;

    for line in BufReader::new(input).lines() {
        let instr: Instruction = line?.parse()?;
    }

    Ok(())
}
