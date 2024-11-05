use wawawa::{Config, run};

use winit::dpi::PhysicalSize;

fn main() {
    let mut args = std::env::args();
    let mut config = Config::default();

    if let Some(arg) = args.nth(1) {
        if arg == "--size" {
            if let Some(size) = args.next() {
                let mut split = size.split(',');
                let width = split.next().unwrap().parse::<u32>().unwrap();
                let height = split.next().unwrap().parse::<u32>().unwrap();
                config.initial_window_size = PhysicalSize { width, height };
            } else {
                eprintln!("Actually write a size bruh");
                std::process::exit(1);
            }
        } else {
            eprintln!("Invalid option {}", arg);
            std::process::exit(1);
        }
    }

    run(config);
}
