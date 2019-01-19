use std::io;

fn main() {
    //program to convert temperature from one scale to another
    //user enters temperature and scale

    println!("Program to convert between Celsius and Fahrenheit");
    
    //standard template to define string and then conver to float
    println!("Enter temperature");
    let mut temp = String::new();

    io::stdin().read_line(&mut temp)
        .expect("Failed to read");
    
    //define floating point temperature
    let temp: f32 = temp.trim().parse()
        .expect("Please type a number");

    //define start_end scale as string
    println!("Enter start end scale: F_C or C_F");
    let mut convert_from_to = String::new();
    io::stdin().read_line(&mut convert_from_to)
        .expect("Failed to read");
    
    //TODO: check which is proper way to set input string to variable
    //options: commented or below
    
    //let convert_from_to: &str = convert_from_to.trim();
    convert_from_to = convert_from_to.trim().to_string();

    println!("You want to convert {}", convert_from_to);
    
    let mut converted_temp: f32 = -1.0;
    
    if convert_from_to == "F_C" {
        println!("Converting from Fahrenheit to Celsius");
        converted_temp = (temp - 32.0)/1.8;
    } else if convert_from_to == "C_F" {
        println!("Converting from Celsius to Fahrenheit");
        converted_temp = 1.8*temp + 32.0;
    } else {
        println!("Please enter proper start end scales!");
    }
    if converted_temp > 0.0 {
        println!("Your temperature {}, converted from {}, is {}",
                 temp,
                 convert_from_to,
                 converted_temp);
    } else {
        println!("Enter proper scale");
    }
}
