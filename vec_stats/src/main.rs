use std::collections::HashMap;
use std::io;

fn vec_mean(v: &[i32]) -> f32 {
    let mut count: f32 = 0.0;
    let mut sum: f32 = 0.0;
    for i in v {
        let i = *i as f32;
        sum += i;
        count += 1.0;
    }
    let avg: f32 = sum / count;
    return avg;
}

fn vec_median(v: &mut Vec<i32>) -> f32 {
    v.sort();
    let mid = v.len() / 2;
    if v.len() % 2 == 0 {
        ((v[mid] + v[mid - 1]) as f32) / 2_f32
    } else {
        v[mid] as f32
    }
}

//Note: If multiple modes, picks the first it finds (Hash maps arent ordered so the first it finds may be unexpected.)
//Not sure how to fix this yet
fn vec_mode(v: &[i32]) -> i32 {
    let mut mode_map = HashMap::new();
    for i in v {
        let count = mode_map.entry(i).or_insert(0);
        *count += 1;
    }
    let mut mode: i32 = 0;
    let mut mode_occurrences: i32 = 0;
    for (element, counter) in &mode_map {
        if *counter > mode_occurrences {
            mode_occurrences = *counter;
            mode = **element;
        }
    }
    return mode;
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    loop {
        println!("Please input a value to add to the integer list:");
        println!("Enter 'quit' when finished");
        let mut inp_val = String::new();

        io::stdin()
            .read_line(&mut inp_val)
            .expect("Failed to read line");

        if inp_val.trim() == "quit" {
            break;
        }

        let inp_val: i32 = match inp_val.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input invalid, try again!");
                continue;
            }
        };
        println! {"You entered: {}",inp_val};
        v.push(inp_val);
    }
    println! {"The list you entered is {:?}",v};

    println! {"Mean: {}",vec_mean(&v)};
    println! {"Median: {}",vec_median(&mut v)};
    println! {"Mode: {}",vec_mode(&v)};
}
