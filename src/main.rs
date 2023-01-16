fn main() {
    let temp: f64 = 70.0;

    let fahrenheit = convert_to_f(temp);
    let celsius = convert_to_c(fahrenheit);

    println!("original temp: {temp}");
    println!("fahrenheit temp: {fahrenheit}");
    println!("celsius temp: {celsius}");
}

fn convert_to_f(temp: f64) -> f64 {
    (temp - 32.0) * (5.0 / 9.0)
}

fn convert_to_c(temp: f64) -> f64 {
    (temp * 9.0 / 5.0) + 32.0
}
