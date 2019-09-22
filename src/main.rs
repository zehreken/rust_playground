extern crate rand;
extern crate sdl2;

use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use std::path::Path;
use std::time::Duration;
mod fps_utils;
mod grid;
pub use crate::grid::grid_config;

const MOORE_DIRECTIONS: [Point; 8] = [
    Point { x: -1, y: -1 },
    Point { x: -1, y: 0 },
    Point { x: -1, y: 1 },
    Point { x: 0, y: 1 },
    Point { x: 1, y: 1 },
    Point { x: 1, y: 0 },
    Point { x: 1, y: -1 },
    Point { x: 0, y: -1 },
];

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct Cell {
    position: Point,
    neighbours: [Point; 8],
    current_state: i32,
    future_state: i32,
    on_count: i32,
}

fn calculate_neighbours(x: i32, y: i32) -> [Point; 8] {
    let neighbours: [Point; 8] = [
        Point {
            x: MOORE_DIRECTIONS[0].x + x,
            y: MOORE_DIRECTIONS[0].y + y,
        },
        Point {
            x: MOORE_DIRECTIONS[1].x + x,
            y: MOORE_DIRECTIONS[1].y + y,
        },
        Point {
            x: MOORE_DIRECTIONS[2].x + x,
            y: MOORE_DIRECTIONS[2].y + y,
        },
        Point {
            x: MOORE_DIRECTIONS[3].x + x,
            y: MOORE_DIRECTIONS[3].y + y,
        },
        Point {
            x: MOORE_DIRECTIONS[4].x + x,
            y: MOORE_DIRECTIONS[4].y + y,
        },
        Point {
            x: MOORE_DIRECTIONS[5].x + x,
            y: MOORE_DIRECTIONS[5].y + y,
        },
        Point {
            x: MOORE_DIRECTIONS[6].x + x,
            y: MOORE_DIRECTIONS[6].y + y,
        },
        Point {
            x: MOORE_DIRECTIONS[7].x + x,
            y: MOORE_DIRECTIONS[7].y + y,
        },
    ];

    return neighbours;
}

fn get_live_neighbour_count(
    cell: Cell,
    grid: [[Cell; grid_config::COLUMN_COUNT as usize]; grid_config::ROW_COUNT as usize],
) -> i32 {
    let mut neighbour_count: i32 = 0;
    for i in 0..8 {
        if cell.neighbours[i].x >= 0
            && cell.neighbours[i].x < grid_config::ROW_COUNT
            && cell.neighbours[i].y >= 0
            && cell.neighbours[i].y < grid_config::COLUMN_COUNT
        {
            let current_state =
                grid[cell.neighbours[i].y as usize][cell.neighbours[i].x as usize].current_state;
            neighbour_count += current_state;
        }
    }

    return neighbour_count;
}

fn cell_tick(
    mut cell: Cell,
    grid: [[Cell; grid_config::COLUMN_COUNT as usize]; grid_config::ROW_COUNT as usize],
) -> Cell {
    let live_neighbour_count: i32 = get_live_neighbour_count(cell, grid);

    if cell.current_state == 1 {
        if live_neighbour_count < 2 {
            cell.future_state = 0;
            cell.on_count = 0;
        } else if live_neighbour_count == 2 || live_neighbour_count == 3 {
            cell.future_state = 1;
            cell.on_count += 1;
        } else {
            cell.future_state = 0;
            cell.on_count = 0;
        }
    } else {
        if live_neighbour_count == 3 {
            cell.future_state = 1;
        } else {
            cell.future_state = 0;
        }
    }

    return cell;
}

fn cell_swap(mut cell: Cell) -> Cell {
    cell.current_state = cell.future_state;

    return cell;
}

fn create_cell() -> Cell {
    let cell = Cell {
        position: Point { x: 0, y: 0 },
        neighbours: calculate_neighbours(0, 0),
        current_state: 0,
        future_state: 0,
        on_count: 0,
    };

    return cell;
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut grid: [[Cell; grid_config::COLUMN_COUNT as usize]; grid_config::ROW_COUNT as usize] =
        [[create_cell(); grid_config::COLUMN_COUNT as usize]; grid_config::ROW_COUNT as usize];

    for row in 0..grid_config::ROW_COUNT {
        for column in 0..grid_config::COLUMN_COUNT {
            grid[row as usize][column as usize].position = Point { x: column, y: row };
            grid[row as usize][column as usize].neighbours = calculate_neighbours(column, row);
            grid[row as usize][column as usize].current_state =
                if rng.gen_range(0, 2) < 1 { 0 } else { 1 };
        }
    }

    /*
    grid[0][0].current_state = 1;
    grid[1][1].current_state = 1;
    grid[2][2].current_state = 1;
    grid[3][3].current_state = 1;
    grid[4][4].current_state = 1;
    grid[1][3].current_state = 1;
    println!(
        "{}, {}, current_state: {}",
        grid[0][0].position.x, grid[0][0].position.y, grid[0][0].current_state
    );
    grid[0][0] = cell_tick(grid[0][0], grid);
    println!("current_state: {}", grid[0][0].current_state);
    */

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(
            "Game",
            grid_config::SCREEN_WIDTH as u32,
            grid_config::SCREEN_HEIGHT as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path = Path::new("emulogic.ttf");
    let font = ttf_context.load_font(font_path, 12).unwrap();
    //   font.set_style(sdl2::ttf::FontStyle::BOLD);

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let surface: Surface = font
        .render("TTF Text Rendering Test")
        .solid(Color::RGBA(0, 0, 0, 255))
        .unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    let text_query = texture.query();
    let text_rect = Rect::new(0, 0, text_query.width, text_query.height);
    println!("{}, {}", text_query.width, text_query.height);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        // Render here
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        for row in 0..grid_config::ROW_COUNT {
            for column in 0..grid_config::COLUMN_COUNT {
                let cell: Cell = grid[row as usize][column as usize];
                if cell.current_state == 1 {
                    let red: u8 = if 20 * cell.on_count > 255 {
                        255
                    } else {
                        20 * cell.on_count as u8
                    };
                    canvas.set_draw_color(Color::RGB(red, 0, 0));
                    let cell_size: i32 = grid_config::CELL_SIZE;
                    canvas.fill_rect(Rect::new(
                        column * cell_size,
                        row * cell_size,
                        cell_size as u32,
                        cell_size as u32,
                    ));
                }
                grid[row as usize][column as usize] =
                    cell_tick(grid[row as usize][column as usize], grid);
            }
        }

        for row in 0..grid_config::ROW_COUNT {
            for column in 0..grid_config::COLUMN_COUNT {
                grid[row as usize][column as usize] =
                    cell_swap(grid[row as usize][column as usize]);
            }
        }

        canvas.copy(&texture, None, text_rect);

        canvas.present();

        ::std::thread::sleep(Duration::from_millis(20));
    }
}
