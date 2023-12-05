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
            2 => days::day_2::solve(),
            3 => days::day_3::solve(),
            4 => days::day_4::solve(),
            5 => days::day_5::solve(),
            _ => println!("Solution for day {} does not exist.", day),
        }
        println!("~~~~~~~~~~~~~~~~~~~~~~~");
    }
    if let Some(day) = args.measure {
        println!("~~~~~~~~~~~~~~~~~~~~~~~");
        println!("Measuring day: {}", day);
        match day {
            1 => measure(days::day_1::solve),
            2 => measure(days::day_2::solve),
            3 => measure(days::day_3::solve),
            4 => measure(days::day_4::solve),
            5 => measure(days::day_5::solve),
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
