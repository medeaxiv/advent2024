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
    print!("Problem {}-{}: ", problem_num, part_idx + 1);
    let result = runner.run();
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
