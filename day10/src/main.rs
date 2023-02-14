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

struct XReg {
    reg: i32,
    pipeline: Vec<Option<i32>>,
    pos: usize
}

impl XReg {
    fn new(init_val: i32, pipeline_length: usize) -> Self {
        Self {
            reg: init_val,
            pipeline: vec![None; pipeline_length],
            pos: 0
        }
    }

    fn tick(&mut self) {
        if let Some(val) = self.pipeline[self.pos] {
            // Add the value
            self.reg += val;
            self.pipeline[self.pos] = None;
        }

        // Increment pos
        self.pos = (self.pos + 1) % self.pipeline.len();
    }

    fn issue_add(&mut self, val : i32) {
        let plen = self.pipeline.len();
        let last_idx = (self.pos + (plen - 1)) % plen;
        self.pipeline[last_idx] = Some(val);
    }

    fn get_curr_val(&self) -> i32 {
        self.reg
    }
}



fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let input = File::open(input)?;

    // There is a 2 cycle delay before values are updated.
    // Values are updated 'between' cycles though, so we
    // insert an extra delay
    const PIPELINE_LEN: usize = 2;

    let mut xreg = XReg::new(1, PIPELINE_LEN);
    let mut cycle : i32 = 1;

    let mut res : i32 = 0;

    let mut capture_result = |val, cycle| {
        res += cycle * val;
    };

    for line in BufReader::new(input).lines() {        
        let instr: Instruction = line?.parse()?;
        let cycles = match instr {
            Instruction::Noop => 1,
            Instruction::Addx(val) => {
                xreg.issue_add(val);
                2
            }
        };

        for _ in 0..cycles {
            if ((cycle + 20) % 40) == 0 {
                capture_result(xreg.get_curr_val(), cycle);
            }

            xreg.tick();
            cycle += 1;
        }
    }

    // Print result:
    println!("Result was {}", res);

    Ok(())
}
