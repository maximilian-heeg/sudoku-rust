use indicatif::*;
use rayon::prelude::*;
use std::env;
use std::fs;
use std::time::*;
use sudoku::*;

fn main() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();

    let file_path = parse_config(&args);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let number_of_puzzles = contents.lines().count();
    println!("Total of {} puzzles... Let's start...", number_of_puzzles);

    let progress_bar = ProgressBar::new(number_of_puzzles.try_into().unwrap());

    contents
        .par_lines()
        .for_each(|line| solve_line(line, &progress_bar));

    progress_bar.finish();
    let elapsed_time = now.elapsed();
    println!(
        "Solving {} Sudokus took {} milliseconds.",
        number_of_puzzles,
        elapsed_time.as_millis()
    );
}

fn solve_line(line: &str, pb: &ProgressBar) {
    let parts = line.split(",").collect::<Vec<&str>>();
    let arr = string_to_sudoku(parts[0]).unwrap();
    let grid = SudokuGrid {
        fields: arr,
        guesses: 0,
        round: 0,
        steps: vec![],
    };
    let solved = grid.solve().unwrap();

    let solution = string_to_sudoku(parts[1]).unwrap();
    assert_eq!(solved.fields, solution);
    pb.inc(1);
}

fn parse_config(args: &[String]) -> &str {
    let file_path = &args[1];

    file_path
}
