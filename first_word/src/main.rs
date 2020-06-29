use std::io;

//Finds the first word of a string slice
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    //Enumerate the string
    for (i, &item) in bytes.iter().enumerate() {
        //When space found, return from start of string up to the space
        if item == b' ' {
            return &s[0..i];
        }
    }
    //Otherwise, only one word (Or empty), return the full input string
    &s[..]
}

//Finds second word of a string slice
fn second_word(s: &str) -> &str {
    //Set up mutable start and end indices
    let mut start_index: usize = 0;
    let mut end_index: usize = 0;
    //Flag to signify if second word found
    let mut second_found: bool = false;

    let bytes = s.as_bytes();

    //Enumerate over the string
    for (i, &item) in bytes.iter().enumerate() {
        //Finds space when second word not found yet
        if item == b' ' && !second_found {
            //Set index points to one after the space
            start_index = i + 1;
            end_index = i + 1;
            //Flag that second word has been found
            second_found = true;
            //Loop to next character
            continue;
        }
        //When second word found
        else if second_found {
            //Check if at space or at end of string (Only two words)
            if item == b' ' || i == s.len() - 1 {
                //Move end_index to end of second word
                end_index += 1;
                //Return the second word
                return &s[start_index..end_index];
            }
            //Otherwise iterate again
            end_index += 1;
        }
    }
    //If no second word, return message informing user
    "There is no second word"
}

fn main() {
    println!("Enter a string:");
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Input reading has failed!");
    //Take off trailing whitespace for the functions
    let input_string = input_string.trim();
    if input_string.is_empty() {
        println!("String is empty!")
    } else {
        let first_word = first_word(input_string);
        let second_word = second_word(input_string);
        println!("The first word is: {}", first_word);
        println!("The second word is: {}", second_word);
    }
}
