mod days;
mod util;

use clap::Parser;

fn main() {
    let args = util::Cli::parse();
    if let Some(day) = args.run {
        println!("~~~~~~~~~~~~~~~~~~~~~~~");
        println!("Solving day: {}", day);
        match day {
            1 => days::day_1::solve(),
            _ => println!("Solution for day {} does not exist.", day),
        }
        println!("~~~~~~~~~~~~~~~~~~~~~~~");
    }
    if let Some(day) = args.measure {
        println!("~~~~~~~~~~~~~~~~~~~~~~~");
        println!("Measuring day: {}", day);
        match day {
            1 => measure(days::day_1::solve),
            _ => println!("Can not measure day {} solution.", day),
        }
        println!("~~~~~~~~~~~~~~~~~~~~~~~");
    }
}

fn measure(f: fn()) {
    let start = std::time::Instant::now();
    f();
    println!("Elapsed: {:?}", start.elapsed());
}
