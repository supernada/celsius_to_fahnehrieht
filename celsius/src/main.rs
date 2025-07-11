use std::io;

fn main() {

    let mut choice = String::new();
    let mut temp_input = String::new();

    println!("Temperature Conversion");
    println!("Enter the temperature value:");

    io::stdin()
        .read_line(&mut temp_input)
        .expect("Something went terribly wront, please reconsider");

    let temp: f64 = match temp_input.trim().parse() {
        Ok(num) => num,
            Err(_) => {
                println!("Jesus, just type a number");
                return;
        }
        
    };

    println!("To Convert C° to F°, press 1");
    println!("To Convert F° to C°, press 2");

    
    io::stdin()
        .read_line(&mut choice)
        .expect("Something went wrong, don't panic");

    let choice = choice.trim();

        
    if choice == "1" {

        let fahrenheit = temp * 1.8 + 32.0;
        println!("{:.2} C° is {:.2} F°", temp, fahrenheit);
        if fahrenheit < -459.67 {
            println!("You know that is impossible, right? I mean, by the current laws of physics. at least.");
        }

    } else if choice == "2"{
        let celsius = (temp - 32.0)/ 1.8;
        println!("{:.2} F° is {:.2} C°", temp, celsius);
        if celsius < -273.415 {
            println!("You know that is impossible, right? I mean, by the current laws of physics, at least.");
        }
    } else {
        println!("Something went different than expected. Maybe your funny '{}'.", choice);
    }
}





