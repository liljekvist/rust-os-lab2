use std::env;
use std::fs::read_to_string;
use std::num::ParseIntError;
use std::str::FromStr;

fn get_command_line_args() -> Result<(usize, u32, String), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        return Err("Arguments missing");
    }
    let num_pages: usize = args[1].parse().unwrap();
    let page_size: u32 = args[2].parse().unwrap();
    let filename = args[3].clone();
    Ok((num_pages, page_size, filename))
}

fn get_file_data(filename: &str) -> Result<Vec<u32>, ParseIntError> {
    read_to_string(filename)
        .unwrap()
        .lines()  // no need for comment since we've extracted this into a function
        .map(u32::from_str)
        .collect()
}

fn main() {
    let (num_pages, page_size, filename) = match get_command_line_args() {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let file_data = match get_file_data(&filename) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("An error occurred while reading file: {}", e);
            return;
        }
    };

    let mut page_array: Vec<u32> = vec![u32::MAX; num_pages];
    let mut next: usize = 0;
    let mut page_fault: u32 = 0;

    for data in &file_data {
        let page: u32 = data / page_size;
        let found: Option<usize> = page_array.iter().position(|&x| x == page);
        if found.is_none() {
            page_fault += 1;
            page_array[next] = page;
            next = (next + 1) % num_pages;
        }
    }

    println!("{} {} {}", num_pages, page_size, filename);
    println!("Number of pagefaults: {}", page_fault);
}