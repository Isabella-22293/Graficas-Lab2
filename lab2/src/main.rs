use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

mod framebuffer;

use crate::framebuffer::Framebuffer;

const WINDOW_WIDTH: usize = 600;  
const WINDOW_HEIGHT: usize = 600; 

const FRAMEBUFFER_WIDTH: usize = 100;  
const FRAMEBUFFER_HEIGHT: usize = 100; 

const FRAME_DELAY: Duration = Duration::from_millis(100);

fn main() {
    let mut window = Window::new(
        "Conway's Game of Life - Press ESC to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut framebuffer = Framebuffer::new(FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT);
    framebuffer.set_background_color(0x000000);

    let initial_pattern = generate_hwss();

    let mut grid = vec![vec![false; FRAMEBUFFER_WIDTH]; FRAMEBUFFER_HEIGHT];
    for &(x, y) in &initial_pattern {
        grid[y][x] = true;
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear();
        update_and_render(&mut grid, &mut framebuffer);
        window
            .update_with_buffer(&framebuffer.buffer, FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT)
            .unwrap();

        std::thread::sleep(FRAME_DELAY);
    }
}

fn generate_hwss() -> Vec<(usize, usize)> {
    let mut pattern = Vec::new();
    let offset_x = 40;  
    let offset_y = 40;  

    let positions = [
        (0, 1), (1, 1), (2, 1), (3, 1), (4, 1),
        (0, 2),                      (4, 2),
        (4, 3),
        (0, 4),                      (3, 4),
                    (1, 5),
    ];

    for &(dx, dy) in &positions {
        pattern.push((offset_x + dx, offset_y + dy));
    }

    pattern
}

fn count_live_neighbors(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    let mut count = 0;
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    for &(dx, dy) in &directions {
        let nx = x.wrapping_add(dx as usize);
        let ny = y.wrapping_add(dy as usize);
        if nx < FRAMEBUFFER_WIDTH && ny < FRAMEBUFFER_HEIGHT && grid[ny][nx] {
            count += 1;
        }
    }

    count
}

fn update_and_render(grid: &mut Vec<Vec<bool>>, framebuffer: &mut Framebuffer) {
    let mut next_grid = grid.clone();

    for y in 0..FRAMEBUFFER_HEIGHT {
        for x in 0..FRAMEBUFFER_WIDTH {
            let live_neighbors = count_live_neighbors(grid, x, y);

            if grid[y][x] {
                if live_neighbors < 2 || live_neighbors > 3 {
                    next_grid[y][x] = false;  
                }
            } else if live_neighbors == 3 {
                next_grid[y][x] = true;  
            }

            if next_grid[y][x] {
                framebuffer.set_current_color(0xFFFFFF); 
                framebuffer.point(x, y);
            }
        }
    }

    *grid = next_grid;
}
