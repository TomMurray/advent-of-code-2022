use std::{
    env,
    error::Error,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

enum Instruction {
    Noop,
    Addx(i32),
}

#[derive(Clone, Debug)]
struct InstructionParseError(String);

impl fmt::Display for InstructionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not parse instruction '{}'", self.0)
    }
}

impl Error for InstructionParseError {}

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
                            Ok(count) => Ok(Instruction::Addx(count)),
                            Err(err) => Err(InstructionParseError(err.to_string())),
                        }
                    } else {
                        Err(InstructionParseError(String::from(s)))
                    }
                }
                _ => Err(InstructionParseError(format!(
                    "Unrecognised instruction '{}'",
                    name
                ))),
            }
        } else {
            Err(InstructionParseError(String::from(s)))
        }
    }
}

struct XReg {
    reg: i32,
    pipeline: Vec<Option<i32>>,
    pos: usize,
}

impl XReg {
    fn new(init_val: i32, pipeline_length: usize) -> Self {
        Self {
            reg: init_val,
            pipeline: vec![None; pipeline_length],
            pos: 0,
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

    fn issue_add(&mut self, val: i32) {
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
    let mut cycle: usize = 1;

    let mut res: i32 = 0;

    let mut capture_result_pt1 = |val, cycle| {
        res += (cycle as i32) * val;
    };

    const CRT_DIMS: [usize; 2] = [6, 40];
    let num_pixels: usize = CRT_DIMS.into_iter().product();

    let mut crt_storage: Vec<char> = Vec::with_capacity(num_pixels);

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
            // Current cycle logic goes here
            let curr_val = xreg.get_curr_val();

            if ((cycle + 20) % 40) == 0 {
                capture_result_pt1(curr_val, cycle);
            }

            // Cycle is 1-based indexing, crt_storage is 0-based
            let cursor_x: i32 = ((cycle - 1) % CRT_DIMS[1]).try_into().unwrap();
            let sprite_x: i32 = curr_val;

            println!("cursor_x = {}, sprite_x = {}", cursor_x, sprite_x);

            // See if the sprite position overlaps the cursor position
            if cursor_x >= (sprite_x - 1) && cursor_x <= (sprite_x + 1) {
                crt_storage.push('#');
            } else {
                crt_storage.push('.');
            }

            xreg.tick();
            cycle += 1;
        }
    }

    // Print result:
    println!("Part 1 result was {}", res);

    // Print crt for part 2:
    for (index, c) in crt_storage.into_iter().enumerate() {
        if index != 0 && index % CRT_DIMS[1] == 0 {
            println!("");
        }
        print!("{}", c);
    }

    Ok(())
}
