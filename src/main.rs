use std::fs::{self,File};
use std::io::{Write,Error};


fn create_day(day:u32) -> Result<(),Error>
{
    // make the sub folder for a single day
    let day_folder= format!("src/day{:02}" , day);
    fs::create_dir_all(&day_folder)?;

    // make the input file for the day
    let input_file = format!("{}/input.txt", day_folder);
    File::create(&input_file)?;

    // make a src file module for the day
    let src_path = format!("{}/mod.rs", day_folder);
    let mut src_file = File::create(&src_path)?;

    let start = format!("//PROMPT: https://adventofcode.com/2024/day/{}" , day);
    let parts = "pub fn part1(input: &str) -> String {\"test1\"} \n pub fn part2(input: &str) -> String {\"test2\"}";

    src_file.write_all(format!("{} \n {}", start, parts).as_bytes())?;
    
    Ok(())
}

fn create_main(days: &[u32]) -> Result<(),Error>
{
    let mut main_file= File::create("src/main.rs")?;

    let modules: String= days.iter()
        .map(|&d| format!("mod day{:02};\n" , d))
        .collect();
    let uses: String = days.iter()
        .map(|&d| format!("use day{:02}::{{part1,part2}};\n" , d))
        .collect();

    let tests: String= days.iter().
        map(|&d| {
            format!("println!(Day {} Part 1: {{}}, part1(include_str!(\"day{:02}/input.txt\")));\nprintln!(Day {} Part 2: {{}}, part2(include_str!(\"day{:02}/input.txt\")));",d,d,d,d)
        }).collect();
    
        // Write the entire main.rs file
    main_file.write_all(format!("// Advent of Code 2024 \n {} {} \n fn main() {{ \n println!(\"Advent of Code 2024\"); \n }} \n {}", modules, uses, tests).as_bytes())?;

    // Return Ok if successful
    Ok(())
}

fn setup_entire_project() -> Result<(),Error>
{
    fs::create_dir_all("src")?;

    // Days 1 to 25
    let days: Vec<u32> = (1..=25).collect();
    for &day in &days {
        create_day(day)?;
    }

    create_main(&days)?;
    let mut cargo_file = File::create("Cargo.toml")?;
    cargo_file.write_all(b"[package] \n name = \"advent-of-code-2024\" \n version = \"0.1.0\" \n edition = \"2021\" \n [dependencies]")?;

    println!("Advent of Code 2024 project structure created!");
    println!("START CODING");
    Ok(())
}

fn main() {
    println!("Hello, world!");

    if let Err(e) = setup_entire_project() {
        eprintln!("Error setting up project: {}", e);
        std::process::exit(1);
    }
}
