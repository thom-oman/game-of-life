use rand::Rng;
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Grid {
    pub cells: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            cells: vec![vec![false; width]; height],
            width,
            height,
        }
    }

    pub fn random(width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut grid = Grid::new(width, height);
        for y in 0..height {
            for x in 0..width {
                grid.cells[y][x] = rng.gen_bool(0.3);
            }
        }
        grid
    }

    pub fn set(&mut self, x: usize, y: usize, alive: bool) {
        if x < self.width && y < self.height {
            self.cells[y][x] = alive;
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            self.cells[y][x]
        } else {
            false
        }
    }

    pub fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                    if self.cells[ny as usize][nx as usize] {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn next_generation(&self) -> Grid {
        let mut next = Grid::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.count_neighbors(x, y);
                let alive = self.cells[y][x];
                next.cells[y][x] = match (alive, neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }
        next
    }

    pub fn count_alive(&self) -> usize {
        self.cells
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| cell)
            .count()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.cells {
            for &cell in row {
                write!(f, "{}", if cell { "█" } else { " " })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn glider(grid: &mut Grid, x: usize, y: usize) {
    let pattern = [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
    for (dx, dy) in pattern.iter() {
        grid.set(x + dx, y + dy, true);
    }
}

pub fn blinker(grid: &mut Grid, x: usize, y: usize) {
    for i in 0..3 {
        grid.set(x + i, y, true);
    }
}

pub fn toad(grid: &mut Grid, x: usize, y: usize) {
    for i in 1..=3 {
        grid.set(x + i, y, true);
    }
    for i in 0..3 {
        grid.set(x + i, y + 1, true);
    }
}

pub fn beacon(grid: &mut Grid, x: usize, y: usize) {
    grid.set(x, y, true);
    grid.set(x + 1, y, true);
    grid.set(x, y + 1, true);
    grid.set(x + 3, y + 2, true);
    grid.set(x + 2, y + 3, true);
    grid.set(x + 3, y + 3, true);
}

pub fn pulsar(grid: &mut Grid, x: usize, y: usize) {
    let pattern = [
        (2, 0),
        (3, 0),
        (4, 0),
        (8, 0),
        (9, 0),
        (10, 0),
        (0, 2),
        (5, 2),
        (7, 2),
        (12, 2),
        (0, 3),
        (5, 3),
        (7, 3),
        (12, 3),
        (0, 4),
        (5, 4),
        (7, 4),
        (12, 4),
        (2, 5),
        (3, 5),
        (4, 5),
        (8, 5),
        (9, 5),
        (10, 5),
        (2, 7),
        (3, 7),
        (4, 7),
        (8, 7),
        (9, 7),
        (10, 7),
        (0, 8),
        (5, 8),
        (7, 8),
        (12, 8),
        (0, 9),
        (5, 9),
        (7, 9),
        (12, 9),
        (0, 10),
        (5, 10),
        (7, 10),
        (12, 10),
        (2, 12),
        (3, 12),
        (4, 12),
        (8, 12),
        (9, 12),
        (10, 12),
    ];
    for (dx, dy) in pattern.iter() {
        grid.set(x + dx, y + dy, true);
    }
}
