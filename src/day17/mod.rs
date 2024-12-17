use aoc_utils::{str::StrExt, AocError};
use regex::Regex;
use vm::{Trace, TraceEntry};

use self::vm::Vm;

mod vm;

const INPUT: &str = include_str!("input.txt");

pub fn problem<H>(builder: &mut aoc_utils::problem::ProblemBuilder<H>)
where
    H: aoc_utils::harness::Harness,
{
    builder.add_part(|| part_1(INPUT));
    builder.add_part(|| part_2(INPUT));
}

fn part_1(_input: &str) -> anyhow::Result<String> {
    let (mut vm, program) = parse(_input)?;
    let output = vm.execute_program(&program)?;
    let result = format_output(&output);
    Ok(result)
}

fn format_output(output: &[i64]) -> String {
    let mut string = String::new();

    for (index, value) in output.iter().enumerate() {
        if index != 0 {
            string.push(',');
        }

        string.push_str(&value.to_string());
    }

    string
}

#[allow(dead_code)]
fn print_trace(trace: &[TraceEntry]) {
    for entry in trace {
        println!("{entry}");
    }
}

fn part_2(input: &str) -> anyhow::Result<i64> {
    let (vm, program) = parse(input)?;
    let mut vm = vm;
    let result = find_quine(&mut vm, &program)?;
    Ok(result)
}

fn parse(input: &str) -> anyhow::Result<(Vm, Vec<i64>)> {
    let mut paragraphs = input.paragraphs();
    let vm_input = paragraphs.next().ok_or(AocError::InvalidInput)?;
    let vm = parse_vm(vm_input)?;

    let program_input = paragraphs.next().ok_or(AocError::InvalidInput)?;
    let program = parse_program(program_input)?;

    Ok((vm, program))
}

fn find_quine<T: Trace>(vm: &mut Vm<T>, program: &[i64]) -> anyhow::Result<i64> {
    // Actually just shamelessly stolen from reddit
    fn recurse<T: Trace>(
        vm: &mut Vm<T>,
        program: &[i64],
        n: i64,
        digit: usize,
    ) -> anyhow::Result<i64> {
        let mut result = vec![i64::MAX];

        for i in 0..8 {
            let nn = n + i * (8i64).pow(digit as u32);

            vm.reset();
            vm.set_registers([nn, 0, 0]);
            let output = vm.execute_program(program)?;

            if output.len() != program.len() {
                continue;
            }

            if output[digit] == program[digit] {
                if digit == 0 {
                    return Ok(nn);
                }

                let next = recurse(vm, program, nn, digit - 1)?;
                result.push(next);
            }
        }

        let result = result.into_iter().min().unwrap();
        Ok(result)
    }

    recurse(vm, program, 0, 15)
}

fn parse_vm(input: &str) -> anyhow::Result<Vm> {
    let regex = Regex::new(r"[0-9]+").unwrap();

    let mut registers = [0; 3];
    for (index, r#match) in regex.find_iter(input).enumerate() {
        if index >= registers.len() {
            return Err(AocError::InvalidInput.into());
        }

        let part = r#match.as_str();
        let value = i64::from_str_radix(part, 10)?;
        registers[index] = value;
    }

    let mut vm = Vm::new();
    vm.set_registers(registers);
    Ok(vm)
}

fn parse_program(input: &str) -> anyhow::Result<Vec<i64>> {
    if let Some(input) = input.trim().strip_prefix("Program: ") {
        let mut program = Vec::new();
        for part in input.split(',') {
            let value = i64::from_str_radix(part, 10)?;
            program.push(value);
        }

        Ok(program)
    } else {
        Err(AocError::InvalidInput.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example.1.txt");

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE_1).unwrap();
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part_2() {
        let _result = part_2(EXAMPLE_1).unwrap();
    }
}
