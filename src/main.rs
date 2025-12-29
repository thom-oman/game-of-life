mod game;
mod config;

use game::*;
use config::*;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let config = get_terminal_config();
    run_terminal_mode(config);
}

fn run_terminal_mode(config: Config) {
    let mut grid = initialize_grid(&config);
    let mut generation = 0;

    loop {
        clear_screen();
        println!(
            "Generation: {} | Living cells: {}",
            generation,
            grid.count_alive()
        );
        println!("{}", grid);

        let next = grid.next_generation();
        if next == grid {
            println!("Simulation reached a stable state. Press Ctrl+C to exit.");
            thread::sleep(Duration::from_secs(5));
            break;
        }
        grid = next;
        generation += 1;

        thread::sleep(Duration::from_millis(100));
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn initialize_grid(config: &Config) -> Grid {
    let width = config.width;
    let height = config.height;

    match config.pattern {
        Pattern::Random => Grid::random(width, height),
        Pattern::Gliders => {
            let mut g = Grid::new(width, height);
            glider(&mut g, 5, 5);
            glider(&mut g, 15, 10);
            glider(&mut g, 25, 5);
            g
        }
        Pattern::Oscillators => {
            let mut g = Grid::new(width, height);
            blinker(&mut g, 10, 10);
            toad(&mut g, 20, 10);
            beacon(&mut g, 30, 10);
            g
        }
        Pattern::Pulsar => {
            let mut g = Grid::new(width, height);
            pulsar(&mut g, 30, 10);
            g
        }
    }
}
