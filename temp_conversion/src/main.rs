use std::io;

fn temp_conversion(temp_cels: &f32) -> f32 {
    return (*temp_cels * 1.8) + 32.0;
}

fn main() {
    println!("Enter the temperature in celsius:");
    let mut temp_cels = String::new();
    io::stdin()
        .read_line(&mut temp_cels)
        .expect("Failed to read line");

    println!("You entered: {} degrees celsius", temp_cels.trim());

    let temp_cels: f32 = temp_cels.trim().parse().expect("Not a number!");
    let temp_fahr: f32 = temp_conversion(&temp_cels);
    println!("That is: {} degrees fahrenheit", temp_fahr);
}
