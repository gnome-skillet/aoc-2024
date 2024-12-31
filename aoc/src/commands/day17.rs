use std::path::PathBuf;

use clap::Parser;

use env_logger;
use log::{debug, info};
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::character::complete::space1;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;
use std::fs;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day17 {
    #[clap(long, short)]
    input: PathBuf,
}

fn my_digit(input: &str) -> IResult<&str, i32> {
    let (input, digits) = digit1(input)?;
    let x: i32 = digits.parse::<i32>().unwrap();
    Ok((input, x))
}

#[derive(Debug, Copy, Clone, Hash, PartialEq)]
pub enum Register {
    A(i32),
    B(i32),
    C(i32),
}

impl Register {
    pub fn new(c: char, d: i32) -> Self {
        match c {
            'A' => Register::A(d),
            'B' => Register::B(d),
            'C' => Register::C(d),
            //_ => None,
            _ => panic!(),
        }
    }

    pub fn value(&self) -> Option<i32> {
        match self {
            Register::A(d) => Some(*d),
            Register::B(d) => Some(*d),
            Register::C(d) => Some(*d),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq)]
pub enum Command {
    ADV(i32),
    BXL(i32),
    BST(i32),
    JNZ(i32),
    BXC(i32),
    OUT(i32),
    BDV(i32),
    CDV(i32),
}

impl Command {
    pub fn new(opcode: i32, d: i32) -> Self {
        match opcode {
            0 => Command::ADV(d),
            1 => Command::BXL(d),
            2 => Command::BST(d),
            3 => Command::JNZ(d),
            4 => Command::BXC(d),
            5 => Command::OUT(d),
            6 => Command::BDV(d),
            7 => Command::CDV(d),
            _ => panic!(),
        }
    }

    pub fn value(&self) -> Option<i32> {
        match self {
            Command::ADV(d)
            | Command::BXL(d)
            | Command::BST(d)
            | Command::JNZ(d)
            | Command::BXC(d)
            | Command::OUT(d)
            | Command::BDV(d)
            | Command::CDV(d) => Some(*d),
        }
    }
}

pub struct Computer {
    register_a: Register,
    register_b: Register,
    register_c: Register,
    jumped: bool,
    instruction_pointer: usize,
    printed_list: Vec<i32>,
    reverse_list: Vec<(i32, Register)>,
}

impl Computer {
    pub fn new(register_a: Register, register_b: Register, register_c: Register) -> Self {
        let printed_list: Vec<i32> = Vec::new();
        let reverse_list: Vec<(i32, Register)> = Vec::new();
        Self {
            register_a,
            register_b,
            register_c,
            jumped: false,
            instruction_pointer: 0usize,
            printed_list,
            reverse_list,
        }
    }
}

impl Computer {
    pub fn combo_operand(&mut self, operand: i32) -> i32 {
        match operand {
            0..=3 => operand,
            4 => {
                self.reverse_list.push((operand, self.register_a));
                self.register_a.value().unwrap()
            }
            5 => {
                self.reverse_list.push((operand, self.register_b));
                self.register_b.value().unwrap()
            }
            6 => {
                self.reverse_list.push((operand, self.register_b));
                self.register_c.value().unwrap()
            }
            _ => panic!(),
        }
    }

    pub fn adv(&mut self, operand: i32) {
        let numerator: f32 = self.register_a.value().unwrap() as f32;
        let combo: i32 = self.combo_operand(operand);
        let denominator: f32 = 2.0_f32.powi(combo);
        let tmp: f32 = numerator / denominator;
        let result: i32 = tmp.floor() as i32;
        info!("adv({operand}) {:?} = {result}", self.register_a);
        self.register_a = Register::A(result);
    }

    pub fn bxl(&mut self, operand: i32) {
        let lhs: i32 = self.register_b.value().unwrap();
        let result: i32 = lhs ^ operand;
        info!("bxl {operand}: register {:?} = {lhs} xor {operand} = {result}", self.register_b);
        self.register_b = Register::B(result);
    }

    pub fn bst(&mut self, operand: i32) {
        let combo: i32 = self.combo_operand(operand);
        let result: i32 = combo % 8i32;
        info!("bst: set {:?} to combo({operand}):{combo} mod 8 = {result}", self.register_b);
        self.register_b = Register::B(result);
    }

    pub fn jnz(&mut self, operand: i32) {
        let rega_value: i32 = self.register_a.value().unwrap();
        if rega_value == 0 {
            info!("jnz op: {operand} - no jump");
            return;
        }
        let combo: i32 = operand;
        self.jumped = true;
        info!("jnz op: {operand}, reg: {rega_value}, instruction pointer jumped to {combo}");
        self.instruction_pointer = combo as usize;
    }

    pub fn bxc(&mut self, _operand: i32) {
        let regb_value: i32 = self.register_b.value().unwrap();
        let regc_value: i32 = self.register_c.value().unwrap();
        let result: i32 = regb_value ^ regc_value;
        info!("bxc {_operand} - regb:{regb_value} xor regc:{regc_value} = {result}");
        self.register_b = Register::B(result);
    }

    pub fn out(&mut self, operand: i32) {
        let combo: i32 = self.combo_operand(operand);
        let result: i32 = combo % 8i32;
        info!("out {operand} - combo {combo} - output {result}");
        self.printed_list.push(result);
    }

    pub fn bdv(&mut self, operand: i32) {
        let numerator: f32 = self.register_a.value().unwrap() as f32;
        let combo: i32 = self.combo_operand(operand);
        let denominator: f32 = 2.0_f32.powi(combo);
        let tmp: f32 = numerator / denominator;
        let result: i32 = tmp.floor() as i32;
        info!("bdv({operand}) {:?} = {result}", self.register_b);
        self.register_b = Register::B(result);
    }

    pub fn cdv(&mut self, operand: i32) {
        let numerator: f32 = self.register_a.value().unwrap() as f32;
        let combo: i32 = self.combo_operand(operand);
        let denominator: f32 = 2.0_f32.powi(combo);
        let tmp: f32 = numerator / denominator;
        let result: i32 = tmp.floor() as i32;
        info!("cdv({operand}) {:?} = {result}", self.register_c);
        self.register_c = Register::C(result);
    }

    pub fn print_output(&self) {
        for i in self.printed_list.iter() {
            print!("{i},");
        }
    }

    pub fn print_reverse(&self) {
        for i in self.reverse_list.iter() {
            print!("{:?},", i);
        }
    }

    pub fn run_command(&mut self, command: Command) {
        //info!(
        //    "run {:?} - registers {:?}, {:?}, {:?}",
        //    command, self.register_a, self.register_b, self.register_c
        //);
        match command {
            Command::ADV(d) => self.adv(d),
            Command::BXL(d) => self.bxl(d),
            Command::BST(d) => self.bst(d),
            Command::JNZ(d) => self.jnz(d),
            Command::BXC(d) => self.bxc(d),
            Command::OUT(d) => self.out(d),
            Command::BDV(d) => self.bdv(d),
            Command::CDV(d) => self.cdv(d),
        }
        if self.jumped {
            self.jumped = false;
            return;
        }
        self.instruction_pointer += 1;
    }

    pub fn run_commands(&mut self, commands: Vec<Command>) {
        while self.instruction_pointer < commands.len() {
            let command: Command = commands[self.instruction_pointer];
            self.run_command(command);
        }
        info!("registers {:?}, {:?}, {:?}", self.register_a, self.register_b, self.register_c);
    }
}

fn parse_register(input: &str) -> IResult<&str, Register> {
    debug!("parse_register: {input}");
    let (input, _) = tag("Register")(input)?;
    let (input, _) = space1(input)?;
    let (input, register) = terminated(one_of("ABC"), tag(":"))(input)?;
    let (input, _) = space1(input)?;
    let (input, number) = my_digit(input)?;
    Ok((input, Register::new(register, number)))
}

fn parse_registers(input: &str) -> IResult<&str, Vec<Register>> {
    debug!("parse_registers: {input}");
    let (input, registers) = separated_list1(line_ending, parse_register)(input)?;
    Ok((input, registers))
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    debug!("parse_command: {input}");
    let (input, command) = separated_pair(my_digit, tag(","), my_digit)(input)?;
    Ok((input, Command::new(command.0, command.1)))
}

fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    debug!("parse_commands: {input}");
    let (input, _) = tag("Program:")(input)?;
    let (input, _) = space1(input)?;
    let (input, commands) = separated_list1(tag(","), parse_command)(input)?;
    Ok((input, commands))
}

fn parse_challenge(input: &str) -> IResult<&str, (Vec<Register>, Vec<Command>)> {
    debug!("parse_challenge: {input}");
    let (input, registers) = parse_registers(input)?;
    let (input, _) = many1(line_ending)(input)?;
    let (input, commands) = parse_commands(input)?;
    Ok((input, (registers, commands)))
}

impl CommandImpl for Day17 {
    fn main(&self) -> Result<(), DynError> {
        env_logger::init();
        debug!("main");
        let blob_string = fs::read_to_string(&self.input)?;
        match parse_challenge(&blob_string) {
            Ok((_, (registers, commands))) => {
                info!("commands: {:?}", commands);
                let mut computer: Computer =
                    Computer::new(registers[0], registers[1], registers[2]);
                computer.run_commands(commands);
                computer.print_output();
                println!();
                computer.print_reverse();
            }
            Err(error) => panic!("Problem opening the file: {error:?}"),
        };

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_command() -> Result<(), Box<dyn std::error::Error>> {
        let input: &str = "4,2";
        let (_, command) = parse_command(input)?;
        assert_eq!(command, Command::BXC(2i32));
        Ok(())
    }

    #[test]
    fn test_parse_register() -> Result<(), Box<dyn std::error::Error>> {
        let input: &str = "Register A: 30";
        let (_, register) = parse_register(input)?;
        assert_eq!(register, Register::A(30));
        Ok(())
    }
}
