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
            6 => days::day_6::solve(),
            7 => days::day_7::solve(),
            8 => days::day_8::solve(),
            9 => days::day_9::solve(),
            10 => days::day_10::solve(),
            11 => days::day_11::solve(),
            12 => days::day_12::solve(),
            13 => days::day_13::solve(),
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
            6 => measure(days::day_6::solve),
            7 => measure(days::day_7::solve),
            8 => measure(days::day_8::solve),
            9 => measure(days::day_9::solve),
            10 => measure(days::day_10::solve),
            11 => measure(days::day_11::solve),
            12 => measure(days::day_12::solve),
            13 => measure(days::day_13::solve),
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
