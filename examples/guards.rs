// a match guard can be added to filter the arm

enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn main() {
    let temperature = Temperature::Celsius(20);

    match temperature {
        // if introduces a match guard
        Temperature::Celsius(t) if t > 30 => println!("{}C it's damn hot", t),
        Temperature::Celsius(t) => println!("Temperature below 30C it's {}C", t),
        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 F", t),
        Temperature::Fahrenheit(t) => println!("Temeprature below 86F it's {}F", t),
    }
}
