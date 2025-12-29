use game_of_life::game::*;
use game_of_life::config;
use macroquad::prelude::*;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let config = config::get_window_config();
    let mut grid = initialize_grid(&config);

    // Configure window
    let window_width = (config.width as u32 * config.cell_size) as f32;
    let window_height = (config.height as u32 * config.cell_size + 70) as f32;
    request_new_screen_size(window_width, window_height);

    let cell_size = config.cell_size as f32;
    let mut generation = 0;
    let mut last_update = get_time();
    let update_interval = 0.1;
    let mut is_stable = false;

    loop {
        // Update game state every 100ms (10 FPS)
        if get_time() - last_update >= update_interval && !is_stable {
            let next = grid.next_generation();
            if next == grid {
                is_stable = true;
            }
            grid = next;
            generation += 1;
            last_update = get_time();
        }

        // Render every frame
        clear_background(BLACK);

        // Draw stats
        let stats_text = format!(
            "Generation: {} | Living cells: {}",
            generation,
            grid.count_alive()
        );
        draw_text(&stats_text, 10.0, 20.0, 20.0, WHITE);

        if is_stable {
            draw_text(
                "STABLE STATE REACHED - Press ESC to exit",
                10.0,
                45.0,
                20.0,
                YELLOW,
            );
        } else {
            draw_text("Press ESC to exit", 10.0, 45.0, 20.0, GRAY);
        }

        // Draw grid (each cell as a rectangle)
        let y_offset = 60.0;
        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.cells[y][x] {
                    draw_rectangle(
                        x as f32 * cell_size,
                        y as f32 * cell_size + y_offset,
                        cell_size,
                        cell_size,
                        WHITE,
                    );
                }
            }
        }

        // Check for exit
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

fn initialize_grid(config: &config::Config) -> Grid {
    use config::Pattern;
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
