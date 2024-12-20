use aoc_utils::{
    harness::{Harness, Runner},
    problem::{Problem, ProblemCollection},
    AocError,
};
use clap::Parser;

fn problems<H>(harness: H) -> ProblemCollection
where
    H: Harness,
{
    ProblemCollection::builder(harness)
        .add_problem(1, advent_2024::day01::problem)
        .add_problem(2, advent_2024::day02::problem)
        .add_problem(3, advent_2024::day03::problem)
        .add_problem(4, advent_2024::day04::problem)
        .add_problem(5, advent_2024::day05::problem)
        .add_problem(6, advent_2024::day06::problem)
        .add_problem(7, advent_2024::day07::problem)
        .add_problem(8, advent_2024::day08::problem)
        .add_problem(9, advent_2024::day09::problem)
        .add_problem(10, advent_2024::day10::problem)
        .add_problem(11, advent_2024::day11::problem)
        .add_problem(12, advent_2024::day12::problem)
        .add_problem(13, advent_2024::day13::problem)
        .add_problem(14, advent_2024::day14::problem)
        .add_problem(15, advent_2024::day15::problem)
        .add_problem(16, advent_2024::day16::problem)
        .add_problem(17, advent_2024::day17::problem)
        .add_problem(18, advent_2024::day18::problem)
        .add_problem(19, advent_2024::day19::problem)
        .add_problem(20, advent_2024::day20::problem)
        .build()
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    aoc_utils::tracing::setup_tracing(advent_2024::AOC_LOG);

    let problems = if args.time {
        problems(aoc_utils::harness::TimingHarness)
    } else {
        problems(aoc_utils::harness::SimpleHarness)
    };

    if let Some(problem_num) = args.problem {
        let problem = problems
            .get(problem_num)
            .ok_or(AocError::NoSuchProblem(problem_num))?;

        run_problem(problem_num, problem);
    } else {
        run_all(&problems);
    }

    Ok(())
}

fn run_all(problems: &ProblemCollection) {
    for (problem_num, problem) in problems.iter() {
        for (part_idx, runner) in problem.iter() {
            run_single(problem_num, part_idx, runner)
        }
    }
}

fn run_problem(problem_num: i32, problem: &Problem) {
    for (part_idx, runner) in problem.iter() {
        run_single(problem_num, part_idx, runner)
    }
}

fn run_single(problem_num: i32, part_idx: usize, runner: &dyn Runner) {
    let result = runner.run();
    print!("Problem {}-{}: ", problem_num, part_idx + 1);
    match result {
        Ok(out) => println!("{out}"),
        Err(e) => println!("{e}"),
    }
}

#[derive(Parser)]
struct Args {
    /// Time the solutions
    #[arg(long)]
    time: bool,

    /// Selects what problem to run
    problem: Option<i32>,
}
