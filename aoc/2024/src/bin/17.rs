advent_of_code::solution!(17);

use itertools::Itertools;
use priority_queue::PriorityQueue;
use rustc_hash::FxHashSet;

// 0=adv, 1=bxl, 2=bst, 3=jnz, 4=bxc, 5=out, 6=bdv, 7=cdv
type OpcodeT = u8;
type RegT = u128;
type Regs = [RegT; 3];
type ProgramT = u8;
type Program = Vec<ProgramT>;
type PointerT = usize;
type OutT = u128;

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

const ADV: OpcodeT = 0;
const BXL: OpcodeT = 1;
const BST: OpcodeT = 2;
const JNZ: OpcodeT = 3;
const BXC: OpcodeT = 4;
const OUT: OpcodeT = 5;
const BDV: OpcodeT = 6;
const CDV: OpcodeT = 7;

fn parse(input: &str) -> Machine {
    let (regs_s, program_s) = input.trim().split_once("\n\n").unwrap();
    let program = program_s
        .split(": ")
        .last()
        .unwrap()
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let regs: Regs = regs_s
        .lines()
        .map(|l| l.split(": ").last().unwrap().parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    Machine::new(program, regs)
}

struct Machine {
    pointer: PointerT,
    regs: Regs,
    program: Program,
    output: Vec<OutT>,
}

impl Machine {
    fn new(program: Program, regs: Regs) -> Self {
        Self {
            pointer: 0,
            regs,
            program,
            output: Vec::new(),
        }
    }

    fn adv(&mut self, operand: ProgramT) {
        self.regs[A] = self.regs[A] >> self.combo_operand(operand);
    }

    fn bxl(&mut self, operand: ProgramT) {
        self.regs[B] = self.regs[B] ^ (operand as RegT);
    }

    fn bst(&mut self, operand: ProgramT) {
        self.regs[B] = self.combo_operand(operand) & 7;
    }

    #[allow(dead_code)]
    fn jnz(&mut self, _: ProgramT) {}

    fn bxc(&mut self, _: ProgramT) {
        self.regs[B] = self.regs[B] ^ self.regs[C];
    }

    fn out(&mut self, operand: ProgramT) {
        self.output.push((self.combo_operand(operand) & 7) as OutT);
    }

    fn bdv(&mut self, operand: ProgramT) {
        self.regs[B] = self.regs[A] >> self.combo_operand(operand);
    }

    fn cdv(&mut self, operand: ProgramT) {
        self.regs[C] = self.regs[A] >> self.combo_operand(operand);
    }

    fn combo_operand(&self, operand: ProgramT) -> RegT {
        match operand {
            0 | 1 | 2 | 3 => operand as RegT,
            4 => self.regs[A],
            5 => self.regs[B],
            6 => self.regs[C],
            _ => unreachable!("combo operand unreachable with value: {}", operand),
        }
    }

    fn run_program(&mut self) {
        while self.pointer < self.program.len() {
            let opcode = self.program[self.pointer];
            let operand = self.program[self.pointer + 1];

            match opcode {
                ADV => self.adv(operand),
                BXL => self.bxl(operand),
                BST => self.bst(operand),
                JNZ => {
                    if self.regs[A] != 0 {
                        self.pointer = operand as PointerT;
                        continue; // we'd have to use an integer type
                    }
                }
                BXC => self.bxc(operand),
                OUT => self.out(operand),
                BDV => self.bdv(operand),
                CDV => self.cdv(operand),
                _ => unreachable!("opcode unreachable with value: {}", opcode),
            }
            self.pointer += 2;
        }
    }

    fn output(&self) -> OutT {
        self.output.iter().join("").parse().unwrap()
    }
}

pub fn part_one(input: &str) -> Option<OutT> {
    let mut machine = parse(input);
    machine.run_program();

    Some(machine.output())
}

// Program: 2,4,1,7,7,5,1,7,4,6,0,3,5,5,3,0
// (2,4): B := A % 8
// (1,7): B := B ^ 7
// (7,5): C := A >> B
// (1,7): B := B ^ 7
// (4,6): B := B ^ C
// (0,3): A := A >> 3
// (5,5): out(B % 8)
// (3,0): if A != 0 goto 0
// Machine: 10e0, total_cycles: 8, output_len: 1
// Machine: 10e1, total_cycles: 15, output_len: 2
// Machine: 10e2, total_cycles: 22, output_len: 3
// Machine: 10e3, total_cycles: 29, output_len: 4
// Machine: 10e4, total_cycles: 36, output_len: 5
// Machine: 10e5, total_cycles: 43, output_len: 6
// Machine: 10e6, total_cycles: 50, output_len: 7
// Machine: 10e7, total_cycles: 57, output_len: 8
// Machine: 10e8, total_cycles: 64, output_len: 9
// Machine: 10e9, total_cycles: 71, output_len: 10
// Machine: 10e10, total_cycles: 85, output_len: 12
// Machine: 10e11, total_cycles: 92, output_len: 13
// Machine: 10e12, total_cycles: 99, output_len: 14
// Machine: 10e13, total_cycles: 106, output_len: 15
// Machine: 10e14, total_cycles: 113, output_len: 16
// Machine: 10e15, total_cycles: 120, output_len: 17
// Machine: 10e16, total_cycles: 127, output_len: 18
pub fn part_two(input: &str) -> Option<RegT> {
    let machine = parse(input);
    let program_len = machine.program.len();

    let mut pq = PriorityQueue::new();
    let mut seen = FxHashSet::default();
    pq.push(0, 0);

    while let Some((curr_a, curr_idx)) = pq.pop() {
        seen.insert((curr_a, curr_idx));

        if curr_idx == program_len {
            return Some(curr_a);
        }

        // This works only for my specific program, so testing is skipped
        for b in 0..8 {
            let a = curr_a << 3 | b;
            let b = b ^ 7;
            let c = a >> b;
            let b = b ^ 7;
            let b = b ^ c;

            if ((b % 8) as ProgramT) == machine.program[program_len - (curr_idx + 1)]
                && !seen.contains(&(a, curr_idx + 1))
            {
                pq.push(a, curr_idx + 1);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4635635210));
    }

    // #[test]
    //     fn test_part_two() {
    //         let pt2_example = "
    // Register A: 2024
    // Register B: 0
    // Register C: 0
    //
    // Program: 0,3,5,4,3,0";
    //         let result = part_two(pt2_example);
    //         // assert_eq!(result, Some(117440));
    //     }
}
