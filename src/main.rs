use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Please provide 3 arguments: <number> <from-unit> <to-unit>");
        return;
    }

    let input = &args[1];

    if let Ok(num) = input.parse::<f64>() {
        convert(&num, &args[2], &args[3])
    } else {
        println!("This is not a number");
    }
}

fn convert(number: &f64, from: &str, to: &str) {
    let length_list = ["mm", "cm", "in", "dm", "ft", "yd", "m", "km", "mi"];
    let weight_list = ["mg", "g", "kg", "t", "oz", "lb", "st", "cwt"];

    if length_list.iter().any(|&x| x == from) && length_list.iter().any(|&x| x == to) {
        length(number, from, to);
    } else if weight_list.iter().any(|&x| x == from) && weight_list.iter().any(|&x| x == to) {
        weight(number, from, to);
    } else {
        println!("Unknown unit")
    }
}

fn length(number: &f64, from: &str, to: &str) {
    let mut length_units: HashMap<&str, f64> = HashMap::new();
    length_units.insert("mm", 0.001);
    length_units.insert("cm", 0.01);
    length_units.insert("in", 0.0254);
    length_units.insert("dm", 0.1);
    length_units.insert("ft", 0.3048);
    length_units.insert("yd", 0.9144);
    length_units.insert("m", 1.0);
    length_units.insert("km", 1000.0);
    length_units.insert("mi", 1609.344);

    if let (Some(&from_factor), Some(&to_factor)) = (length_units.get(from), length_units.get(to)) {
        let converted = number * from_factor / to_factor;
        println!("{} {} = {} {}", number, from, converted, to);
    }
}

fn weight(number: &f64, from: &str, to: &str) {
    let mut length_units: HashMap<&str, f64> = HashMap::new();
    length_units.insert("mg", 0.000001);
    length_units.insert("g", 0.001);
    length_units.insert("kg", 1.0);
    length_units.insert("t", 1000.0);
    length_units.insert("oz", 0.0283495);
    length_units.insert("lb", 0.453592);
    length_units.insert("st", 6.35029);
    length_units.insert("cwt", 50.8023);

    if let (Some(&from_factor), Some(&to_factor)) = (length_units.get(from), length_units.get(to)) {
        let converted = number * from_factor / to_factor;
        println!("{} {} = {} {}", number, from, converted, to);
    }
}
