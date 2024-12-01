use std::fs::{self,File};
use std::io::{Write,Error};


fn create_day(day:u32) -> Result<(),Error> {
    // make the sub folder for a single day
    let day_folder= format!("src/days/day{:02}" , day);
    fs::create_dir_all(&day_folder)?;

    // make the input file for the day
    create_file(&format!("{}/input.txt", day_folder), "")?;

    // make a src file module for the day
    create_file(&format!("{}//mod.rs", day_folder), &generate_mod_contents(day))?;

    // files for each part to separate the code
    create_file(&format!("{}/part1.rs", day_folder), &generate_part_contents())?;
    create_file(&format!("{}/part2.rs", day_folder), &generate_part_contents())?;

    Ok(())
}

fn create_file(path: &str, content: &str) -> Result<File, std::io::Error> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(file)
}

fn generate_mod_contents(day:u32) -> String {
    
    let start = format!("use crate::utils; \n//PROMPT: https://adventofcode.com/2024/day/{} \n" , day);
    let mods = format!("mod part1; \n mod part2; \n");
    let function_body = format!("
     let part1_result = part1::solve(include_str!(\"input.txt\"));
     let part2_result = part2::solve(include_str!(\"input.txt\"));
     format!(\"Day {{}} Solutions:\n  Part 1: {{}}\n  Part 2: {{}}\", {}, part1_result, part2_result)
     ", day);
    let generic_solve = format!("pub fn solve()->String {{ {} }}", function_body);

    format!("{} \n {} \n {}", start, mods, generic_solve)
}

fn generate_part_contents() -> String {
    format!("use crate::utils; \npub fn solve(input: &str) -> String{{ \"unsolved\" }}")
}

fn create_main(days: &[u32]) -> Result<(),Error> {
    let mut main_file= File::create("src/main.rs")?;

   
    // Write the entire main.rs file
    main_file.write_all(format!("// Advent of Code 2024 \npub mod utils; \nmod days; \n \nfn main() {{ \n\tprintln!(\"Advent of Code 2024\");  \n\tprintln!(\"{{}}\", days::d1_solve());\n }} \n").as_bytes())?;

    // Return Ok if successful
    create_file("src/utils.rs" , "")?;


    Ok(())
}

fn create_days_module(days: &[u32]) -> Result<(),Error> {

    let content: Vec<String>= days.iter()
        .map(|&day| {format!("pub mod day{};\npub mod use day{}::solve as d{}_solve;\n",day,day,day)})
        .collect();

    //Shoot me in the head
    let m = content.join("");
    let n =m.as_str();
    
    create_file("src/days/mod.rs", n)?;
    Ok(())
}

fn setup_entire_project() -> Result<(),Error> {
    fs::create_dir_all("src")?;

    // Days 1 to 25
    let days: Vec<u32> = (1..=25).collect();
    for &day in &days {
        create_day(day)?;
    }

    create_main(&days)?;
    create_days_module(&days)?;
    create_file("Cargo.toml" , "[package] \n name = \"advent-of-code-2024\" \n version = \"0.1.0\" \n edition = \"2021\" \n [dependencies]")?;
    
    println!("Advent of Code 2024 project structure created!");
    println!("just delete the exe");
    println!("AND START CODING");

    Ok(())
}

fn main() {
    println!("Hello, world!");

    if let Err(e) = setup_entire_project() {
        eprintln!("Error setting up project: {}", e);
        std::process::exit(1);
    }
}
