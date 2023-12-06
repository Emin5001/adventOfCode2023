use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use regex::Regex;

fn main() -> io::Result<()> {
    let path = "../../inputs/d1p1.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    let num_map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    let reverse_map = HashMap::from([
        ("eno", "1"),
        ("owt", "2"),
        ("eerht", "3"),
        ("ruof", "4"),
        ("evif", "5"),
        ("xis", "6"),
        ("neves", "7"),
        ("thgie", "8"),
        ("enin", "9"),
    ]);

    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let reversed_re = Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    for result_line in reader.lines() {
        let mut combined_number: String = String::new();
        let line: String = result_line?;
        println!("Line is = {}", line);
        let mut regex_index: i8 = -1;
        let mut regex_number = "";
        let mut int_index: i8 = -1;
        let mut normal_number = '\0';
        if let Some(cap) = re.captures(&line) {
            regex_index = cap.get(1).unwrap().start() as i8;
            regex_number = num_map.get(&cap[1]).unwrap_or(&"");
            //combined_number.push_str(num_map.get(&cap[1]).unwrap_or(&""))
        }
        for (i, c) in line.chars().enumerate(){
             if c.is_numeric() {
                int_index = i as i8;
                normal_number = c;
                break;
            }
        }

        if regex_index >= 0 && int_index >= 0 && regex_index < int_index {
            combined_number.push_str(regex_number);
        } else if regex_index >= 0 && int_index >= 0 && int_index < regex_index {
            combined_number.push(normal_number);
        } else if regex_index >= 0 && int_index < 0 {
            combined_number.push_str(regex_number);
        } else if regex_index < 0 && int_index >= 0 {
            combined_number.push(normal_number);
        } 
        println!("regex_index={}, int_index={}", regex_index, int_index);
        let reversed_line: String = line.chars().rev().collect(); 
        if let Some(cap) = reversed_re.captures(&reversed_line) {
            regex_index = cap.get(1).unwrap().start() as i8;
            regex_number = reverse_map.get(&cap[1]).unwrap_or(&"");
        }
        for (i, c) in line.chars().rev().enumerate() {
            //print!("{} ", c);
            if c.is_numeric() {
                int_index = i as i8;
                normal_number = c;
                break;
            }         
        }
    
        println!("reversed line is = {}", reversed_line);
        //println!("regex_index={}, int_index={}", regex_index, int_index);
        if regex_index >= 0 && int_index >= 0 && regex_index < int_index {
            combined_number.push_str(regex_number);
        } else if regex_index >= 0 && int_index >= 0 && int_index < regex_index {
            combined_number.push(normal_number);
        } else if regex_index >= 0 && int_index < 0 {
            combined_number.push_str(regex_number);
        } else if regex_index < 0 && int_index >= 0 {
            combined_number.push(normal_number);
        }
        println!("combined number is = {}", combined_number);
        match combined_number.parse::<i32>() {
            Ok(number) => {
                sum += number;
            }
            Err(_e) => {
            }
        }
    }

    println!("The sum is = {}", sum);
    Ok(())
}
