use std::io;
use io::Write;

#[derive(Debug)]
enum ConversionType {
    Fahrenheit,
    Celsius
}

fn main() {
    println!("Welcome to the temperature converter!");
    loop {
        let choice = loop {
            println!("\nWhat conversion would you like to perform?");
            println!("Fahrenheit to (C)elsius");
            println!("Celsius to (F)arenheit");
            print!("\nPlease choose: ");
            io::stdout().flush().unwrap();
            
            let mut choice = String::new();
            io::stdin().read_line(&mut choice)
                .expect("An error occured receiving your input.");
            
            match choice.trim().to_ascii_lowercase().as_str() {
                "f" => break ConversionType::Fahrenheit,
                "c" => break ConversionType::Celsius,
                "q" => {
                    println!("Have a nice day!");
                    return;
                },
                &_ => {
                    println!("Please enter a valid selection.");
                    continue;
                }
            };
        };
        print!("Please enter the temperature to convert to {:?}: ", choice);
        io::stdout().flush().unwrap();
    
        let mut temp = String::new();
        io::stdin().read_line(&mut temp)
            .expect("An error occured receiving your input.");
    
        let temp: u32 =  match temp.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Have a nice day!");
                return
            }
        };

        println!("\n{} degrees converts to {} degrees {:?}.", temp, convert_temp(temp, &choice), choice);
    }
}

fn convert_temp(temp: u32, conversion: &ConversionType) -> u32 {
    match conversion {
        ConversionType::Fahrenheit => (temp * 9/5) + 32,
        ConversionType::Celsius =>  (temp - 32) * 5/9
    } 
}

