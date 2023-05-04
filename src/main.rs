use std::io;

fn main() {
    println!("I am a Celsius-to-Farenheit converter");
    println!("Press F to convert to Fahrenheit, C to convert to celsius");
    loop {        
        let mut unit = String::new();        
        io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read input");

        let unit: String = unit.trim().parse().expect("Something went wrong");

        if unit != "F" && unit != "C" && unit != "c" && unit != "f"{
            println!("Invalid unit, try again");
            continue
        }
        get_value(unit);
        break;
    
    }
}

fn get_value(unit: String) {
    if unit == "F" || unit == "f" { println!("Enter the temperature in celsius") };  
    if unit == "C" || unit == "c" { println!("Enter the temperature in Fahrenheit") };

    loop {        
        let mut temperature = String::new();            
        io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read input");
    
        let temperature: f64 = match temperature.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };
        convert_temperature(temperature, unit.clone());
        break;
    }
}

fn convert_temperature(temperature: f64, unit: String) {
    let floating_point_result: f64 = temperature.into();

    if unit == "c" || unit == "C" {
        let result: f64 = (floating_point_result - 32.0) * 5.0 / 9.0; 
        println!("{temperature} degrees in Fahrenheit is {:.2} degrees in Celsius", result)
    } else {        
        let result: f64 = (floating_point_result * 9.0 / 5.0) + 32.0;
        println!("{temperature} degrees in Celsius is {:.2} degrees in Farenheit", result)
    }

}
