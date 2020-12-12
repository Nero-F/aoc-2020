use std::fs;

fn main() {
    let buffer = fs::read_to_string("input").unwrap();
    let input: Vec<&str> = buffer.lines().collect();
    let mut instruction_vec = to_instruction_vec(&input);

    println!("{}", part1(&mut instruction_vec));
    println!("{}", part2(&mut instruction_vec));
}

#[derive(Debug, Clone)]
struct Instruction {
    pub opcode: String,
    pub value: isize,
    pub is_visited: bool
}

impl Instruction {
    fn new(opcode: String, value: isize) -> Instruction {
        Instruction { opcode: opcode, value: value, is_visited: false }
    }
}

fn to_instruction_vec(input: &Vec<&str>) -> Vec<Instruction> {
    let mut instruction_vec: Vec<Instruction> = Vec::new();
    let mut tmp: Vec<&str>;

    for line in input  {
        tmp = line.split(" ").collect();
        instruction_vec
            .push(Instruction::new(tmp[0].to_string(), tmp[1].parse::<isize>().unwrap()))
    }
    instruction_vec
}

fn part1(instruction_vec: &mut Vec<Instruction>) -> isize {
    let mut accumulator: isize = 0;
    let mut ip: usize = 0;
    let mut x: isize = 0;

    loop {
        if instruction_vec[ip].is_visited == true {
            break;
        }
        instruction_vec[ip].is_visited = true;
        x += match &*instruction_vec[ip].opcode {
            "acc" => {
                accumulator += instruction_vec[ip].value;
                1
            },
            "jmp" => instruction_vec[ip].value,
            _ => 1,
        };
        ip = x as usize;
    }
    accumulator
}

fn replace_instr(instruction: &mut Instruction) {
    if instruction.opcode == "jmp" {
        instruction.opcode = "nop".to_string();
    } else if instruction.opcode == "nop" {
        instruction.opcode = "jmp".to_string();
    }
}

fn part2(instruction_vec: &mut Vec<Instruction>) -> isize {
    let mut accumulator: isize;
    let mut ip: isize;
    let mut stack_trace: Vec<isize> =Vec::new();
    let length = instruction_vec.len();
   
    for x in 0..length {
        accumulator = 0;
        ip = 0;
        stack_trace.clear();
        replace_instr(&mut instruction_vec[x]);
        loop { 
            if stack_trace.contains(&ip) == true {
                break;
            }
            stack_trace.push(ip);
            if ip == (length as isize) {
                return accumulator;
            }
            match &*instruction_vec[ip as usize].opcode {
                "acc" => accumulator += instruction_vec[ip as usize].value,
                "jmp" => ip += instruction_vec[ip as usize].value - 1,
                _ => {},
            }
            ip += 1;
        }
        replace_instr(&mut instruction_vec[x]);
    }
    panic!("Error!!");
}

#[test]
fn atest() {
    let test_input = [
        "nop +0",
        "acc +1",
        "jmp +4",
        "acc +3",
        "jmp -3",
        "acc -99",
        "acc +1",
        "jmp -4",
        "acc +6"
    ];
    let mut instruction_vec = to_instruction_vec(&test_input.to_vec());

    assert_eq!(5, part1(&mut instruction_vec));
    assert_eq!(8, part2(&mut instruction_vec));
}
