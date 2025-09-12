use std::collections::HashMap;

fn main() {
    let length_units: HashMap<&str, f64> = [
        ("mm", 0.001),
        ("cm", 0.01),
        ("in", 0.0254),
        ("dm", 0.1),
        ("ft", 0.3048),
        ("yd", 0.9144),
        ("m", 1.0),
        ("km", 1000.0),
        ("mi", 1609.344),
    ]
    .into_iter()
    .collect();

    let weight_units: HashMap<&str, f64> = [
        ("mg", 0.000001),
        ("g", 0.001),
        ("kg", 1.0),
        ("t", 1000.0),
        ("oz", 0.0283495),
        ("lb", 0.453592),
        ("st", 6.35029),
        ("cwt", 50.8023),
    ]
    .into_iter()
    .collect();

    let volume_units: HashMap<&str, f64> =
        [("ml", 0.000001), ("cl", 0.001), ("dl", 1.0), ("l", 1000.0)]
            .into_iter()
            .collect();

    let time_units: HashMap<&str, f64> = [
        ("ns", 1e-9),
        ("us", 1e-6),
        ("ms", 1e-3),
        ("s", 1.0),
        ("min", 60.0),
        ("h", 3600.0),
        ("d", 86400.0),
    ]
    .into_iter()
    .collect();

    let storage_units: HashMap<&str, f64> = [
        ("kib", 1024.0),
        ("mib", 1024.0 * 1024.0),
        ("gib", 1024.0 * 1024.0 * 1024.0),
        ("tib", 1099511627776.0),
        ("b", 1.0),
        ("kb", 1000.0),
        ("mb", 1_000_000.0),
        ("gb", 1_000_000_000.0),
        ("tb", 1_000_000_000_000.0),
    ]
    .into_iter()
    .collect();
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage: {} <number <from unit> <to unit>", args[0]);
        return;
    }

    let input = &args[1];

    if let Ok(num) = input.parse::<f64>() {
        convert(
            &num,
            &args[2],
            &args[3],
            &length_units,
            &weight_units,
            &volume_units,
            &time_units,
            &storage_units,
        )
    } else {
        println!("This is not a number");
    }
}

fn convert(
    number: &f64,
    from: &str,
    to: &str,
    length_list: &HashMap<&str, f64>,
    weight_list: &HashMap<&str, f64>,
    volume_list: &HashMap<&str, f64>,
    time_list: &HashMap<&str, f64>,
    storage_list: &HashMap<&str, f64>,
) {
    if length_list.contains_key(from) && length_list.contains_key(to) {
        length(number, from, to, length_list);
    } else if weight_list.contains_key(from) && weight_list.contains_key(to) {
        weight(number, from, to, weight_list);
    } else if volume_list.contains_key(from) && volume_list.contains_key(to) {
        volume(number, from, to, time_list);
    } else if time_list.contains_key(from) && time_list.contains_key(to) {
        time(number, from, to, time_list);
    } else if storage_list.contains_key(from) && storage_list.contains_key(to) {
        storage(number, from, to, storage_list);
    } else {
        println!("Unknown unit")
    }
}

fn length(number: &f64, from: &str, to: &str, length_list: &HashMap<&str, f64>) {
    if let (Some(&from_factor), Some(&to_factor)) = (length_list.get(from), length_list.get(to)) {
        let converted = number * from_factor / to_factor;
        println!("{} {} = {} {}", number, from, converted, to);
    }
}

fn weight(number: &f64, from: &str, to: &str, weight_list: &HashMap<&str, f64>) {
    if let (Some(&from_factor), Some(&to_factor)) = (weight_list.get(from), weight_list.get(to)) {
        let converted = number * from_factor / to_factor;
        println!("{} {} = {} {}", number, from, converted, to);
    }
}

fn volume(number: &f64, from: &str, to: &str, volume_list: &HashMap<&str, f64>) {
    if let (Some(&from_factor), Some(&to_factor)) = (volume_list.get(from), volume_list.get(to)) {
        let converted = number * from_factor / to_factor;
        println!("{} {} = {} {}", number, from, converted, to);
    }
}
fn time(number: &f64, from: &str, to: &str, time_list: &HashMap<&str, f64>) {
    if let (Some(&from_factor), Some(&to_factor)) = (time_list.get(from), time_list.get(to)) {
        let converted = number * from_factor / to_factor;
        println!("{} {} = {} {}", number, from, converted, to);
    }
}
fn storage(number: &f64, from: &str, to: &str, storage_list: &HashMap<&str, f64>) {
    if let (Some(&from_factor), Some(&to_factor)) = (storage_list.get(from), storage_list.get(to)) {
        let converted = number * from_factor / to_factor;
        println!("{} {} = {} {}", number, from, converted, to);
    }
}
