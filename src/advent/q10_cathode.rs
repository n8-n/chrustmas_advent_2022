use crate::common::grid::Grid;
use crate::common::io;

const CYCLES_PER_ROW: usize = 40;
const SCREEN_ROWS: usize = 6;

pub fn parse_instructions(filename: &str) -> Vec<Instruction> {
    let lines = io::read_file_as_vector(filename).expect("Could not read file");
    lines
        .iter()
        .map(|s| parse_line_to_instruction(&s))
        .collect()
}

pub fn get_sum_of_signal_strengths(instructions: &Vec<Instruction>) -> i32 {
    let mut cycles_to_check = vec![20, 60, 100, 140, 180, 220];
    let mut cycles_values: Vec<i32> = Vec::with_capacity(cycles_to_check.len());

    instructions
        .iter()
        .fold((1, 0), |(regx, cycles): (i32, i32), instruction| {
            if cycles_to_check.is_empty() {
                return (regx, cycles);
            }

            for i in 1..=instruction.cycles() {
                let current_cycle = cycles + i;

                if current_cycle == cycles_to_check[0] {
                    let strength = regx * cycles_to_check[0];
                    cycles_values.push(strength);
                    cycles_to_check = cycles_to_check[1..].to_vec();
                    break;
                }
            }

            (regx + instruction.value(), cycles + instruction.cycles())
        });

    cycles_values.iter().sum()
}

pub fn print_to_screen(instructions: &Vec<Instruction>) {
    let mut screen: Grid<char> = Grid::new().with_column_size(CYCLES_PER_ROW);
    let mut ins_iter = instructions.iter().peekable();
    let mut regx: i32 = 1;
    let mut position = 0;

    while position != SCREEN_ROWS * CYCLES_PER_ROW {
        if ins_iter.peek().is_none() {
            break;
        };

        screen.elements.push(get_pixel(regx, position));
        position += 1;

        let next = ins_iter.next().expect("Should be a value");
        match next {
            Instruction::Addx(x) => {
                screen.elements.push(get_pixel(regx, position));
                position += 1;
                regx += x;
            }
            Instruction::Noop => continue,
        }
    }
    println!("{}", screen);
}

fn get_pixel(regx: i32, position: usize) -> char {
    let row_position = position % CYCLES_PER_ROW;
    let range = (regx - 1)..=(regx + 1);
    let position = row_position as i32;

    if range.contains(&position) {
        return '#';
    }

    '.'
}

fn parse_line_to_instruction(line: &str) -> Instruction {
    if line.trim() == "noop" {
        return Instruction::Noop;
    }

    let (_, reg_add) = line.split_once(' ').expect("Should split");
    let parse = |s: &str| s.trim().parse().expect("Should parse");

    Instruction::Addx(parse(reg_add))
}

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn cycles(&self) -> i32 {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }

    fn value(&self) -> i32 {
        match self {
            Self::Noop => 0,
            Self::Addx(x) => *x,
        }
    }
}

//
//
//
#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        let s = "noop";
        assert_eq!(Instruction::Noop, parse_line_to_instruction(&s));

        let s = "addx -3";
        assert_eq!(Instruction::Addx(-3), parse_line_to_instruction(&s));
    }

    #[test]
    fn test_get_sum_of_strengths() {
        let result = get_sum_of_signal_strengths(&parse_instructions("resources/test/10_cathode.txt"));
        assert_eq!(13140, result);
    }

    #[test]
    fn test_get_pixel() {
        let regx = 10;
        assert_eq!('#', get_pixel(regx, 9));
        assert_eq!('#', get_pixel(regx, 10));
        assert_eq!('#', get_pixel(regx, 11));
        assert_eq!('.', get_pixel(regx, 8));
        assert_eq!('.', get_pixel(regx, 12));
    }

    #[test]
    fn test_print_to_screen() {
        print_to_screen(&parse_instructions("resources/test/10_cathode.txt"));
    }
}
