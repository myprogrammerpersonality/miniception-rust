use simple_logger;
use miniception_rust::minimizers_locations;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let s = "TTAAAATAAAA";
    let w = 3;
    let k = 4;

    let minimizers: Vec<usize> = minimizers_locations(s, w, k);

    println!("Sequence Length: {}", s.len());
    println!("Number of Windows: {}", s.len() - (w + k - 1) + 1);
    println!("Number of Minimizers: {}", minimizers.len());
    println!("Number of Unique Minimizers: {}", minimizers.iter().collect::<std::collections::HashSet<_>>().len());
    println!("{:?}", minimizers);
}
