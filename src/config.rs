use std::io::{self, Write};
use terminal_size::{terminal_size, Width, Height};

#[derive(Debug, Clone)]
pub enum RenderMode {
    Terminal,
    Window,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Random,
    Gliders,
    Oscillators,
    Pulsar,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub render_mode: RenderMode,
    pub pattern: Pattern,
    pub width: usize,
    pub height: usize,
    pub cell_size: u32,
}

fn get_terminal_dimensions() -> (usize, usize) {
    if let Some((Width(w), Height(h))) = terminal_size() {
        let width = w as usize;
        let height = h.saturating_sub(3) as usize;
        (width, height)
    } else {
        (80, 40)
    }
}

fn prompt_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn get_terminal_config() -> Config {
    println!("Conway's Game of Life - Terminal Mode");
    println!("=====================================");
    println!();
    println!("Choose a starting pattern:");
    println!("1. Random");
    println!("2. Gliders");
    println!("3. Oscillators (Blinker, Toad, Beacon)");
    println!("4. Pulsar");
    println!();

    let pattern_input = prompt_input("Enter your choice (1-4): ");
    let pattern_choice: u32 = pattern_input.parse().unwrap_or(1);

    let pattern = match pattern_choice {
        1 => Pattern::Random,
        2 => Pattern::Gliders,
        3 => Pattern::Oscillators,
        4 => Pattern::Pulsar,
        _ => Pattern::Random,
    };

    let (width, height) = get_terminal_dimensions();

    Config {
        render_mode: RenderMode::Terminal,
        pattern,
        width,
        height,
        cell_size: 1,
    }
}

pub fn get_window_config() -> Config {
    println!("Conway's Game of Life - Window Mode");
    println!("===================================");
    println!();
    println!("Window Configuration:");

    let width_input = prompt_input("Enter window width (default 1280): ");
    let window_width: u32 = if width_input.is_empty() {
        1280
    } else {
        width_input.parse().unwrap_or(1280)
    };

    let height_input = prompt_input("Enter window height (default 720): ");
    let window_height: u32 = if height_input.is_empty() {
        720
    } else {
        height_input.parse().unwrap_or(720)
    };

    let cell_size_input = prompt_input("Enter cell size in pixels (1-16, default 4): ");
    let cell_size: u32 = if cell_size_input.is_empty() {
        4
    } else {
        cell_size_input.parse().unwrap_or(4).clamp(1, 16)
    };

    let grid_width = (window_width / cell_size) as usize;
    let grid_height = (window_height / cell_size) as usize;

    println!();
    println!("Grid will be {}x{} cells", grid_width, grid_height);

    if grid_width > 500 || grid_height > 500 {
        println!("Warning: Large grids may impact performance");
    }

    println!();
    println!("Choose a starting pattern:");
    println!("1. Random");
    println!("2. Gliders");
    println!("3. Oscillators (Blinker, Toad, Beacon)");
    println!("4. Pulsar");
    println!();

    let pattern_input = prompt_input("Enter your choice (1-4): ");
    let pattern_choice: u32 = pattern_input.parse().unwrap_or(1);

    let pattern = match pattern_choice {
        1 => Pattern::Random,
        2 => Pattern::Gliders,
        3 => Pattern::Oscillators,
        4 => Pattern::Pulsar,
        _ => Pattern::Random,
    };

    Config {
        render_mode: RenderMode::Window,
        pattern,
        width: grid_width,
        height: grid_height,
        cell_size,
    }
}
