use std::{fs::create_dir, usize};

use raylib::{ffi::GetFrameTime, prelude::*};

const WINDOW_SIZE: Vector2 = Vector2::new(800.0, 800.0);
const GRID_SIZE: Vector2 = Vector2::new(50.0, 50.0);
const CREATURE_SIZE_X: usize = GRID_SIZE.x as usize;
const CREATURE_SIZE_Y: usize = GRID_SIZE.y as usize;
const CELL_SIZE: Vector2 = Vector2::new(WINDOW_SIZE.x / GRID_SIZE.x, WINDOW_SIZE.y / GRID_SIZE.y);

struct Config {
    do_next_frame: bool,
    cycle_counter: u32,
    speed: f32,
    auto_run: bool,
    frame_counter: u32,
}

struct Vector2i {
    x: i32,
    y: i32,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title("Conway's game of life!")
        .build();

    // Every cell is ether alive (true) or dead (false)
    let mut creatures: [[bool; CREATURE_SIZE_Y]; CREATURE_SIZE_X] =
        [(); CREATURE_SIZE_X as usize].map(|_| [(); CREATURE_SIZE_Y as usize].map(|_| false));

    creatures[11][10] = true;
    creatures[12][11] = true;
    creatures[10][12] = true;
    creatures[11][12] = true;
    creatures[12][12] = true;

    creatures[35][27] = true;
    creatures[36][25] = true;
    creatures[36][26] = true;

    let mut config = Config {
        do_next_frame: false,
        cycle_counter: 0,
        speed: 900.0,
        auto_run: false,
        frame_counter: 0,
    };

    // let mut do_next_frame = false;
    // let mut cycle_counter = 0;
    // let mut speed = 900;
    // let mut auto_run = false;
    // let mut frame_counter = 0;
    //
    while !rl.window_should_close() {
        let frame_time = unsafe { GetFrameTime() };
        let mouse_position: Vector2i = Vector2i {
            x: rl.get_mouse_x(),
            y: rl.get_mouse_y(),
        };
        if rl.is_key_pressed(KeyboardKey::KEY_D) {
            config.do_next_frame = true;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            if config.auto_run {
                config.auto_run = false;
            } else {
                config.auto_run = true;
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_S) && config.speed >= 1.0 {
            config.speed -= 10.0 * frame_time;
        }
        if rl.is_key_down(KeyboardKey::KEY_W) {
            config.speed += 10.0 * frame_time;
        }
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            creatures[(mouse_position.x / CELL_SIZE.x as i32) as usize]
                [(mouse_position.y / CELL_SIZE.y as i32) as usize] = !creatures
                [(mouse_position.x / CELL_SIZE.x as i32) as usize]
                [(mouse_position.y / CELL_SIZE.y as i32) as usize];
        }
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            creatures = [(); CREATURE_SIZE_X as usize]
                .map(|_| [(); CREATURE_SIZE_Y as usize].map(|_| false));
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        // --- Draw ---
        draw_creatures(&mut d, &creatures);
        draw_grid(&mut d);

        d.draw_text(
            &format!("Life cycles: {}", config.cycle_counter),
            10,
            10,
            30,
            Color::BLACK,
        );
        d.draw_text(
            &format!("Autorun speed: {}", config.speed),
            10,
            40,
            30,
            Color::BLACK,
        );
        d.draw_text(
            &format!(
                "X: {} Y: {}",
                mouse_position.x / CELL_SIZE.x as i32,
                mouse_position.y / CELL_SIZE.y as i32
            ),
            10,
            40 + 30,
            30,
            Color::BLACK,
        );

        if !config.do_next_frame && !config.auto_run {
            continue;
        }

        if config.frame_counter <= config.speed as u32 && config.auto_run {
            config.frame_counter += 1;
            continue;
        }

        config.frame_counter = 0;

        // --- Update ---
        let mut new_array: [[bool; CREATURE_SIZE_Y]; CREATURE_SIZE_X] =
            [(); CREATURE_SIZE_X as usize].map(|_| [(); CREATURE_SIZE_Y as usize].map(|_| false));

        update_creatures(&creatures, &mut new_array);
        creatures = new_array;

        config.cycle_counter += 1;
        config.do_next_frame = false;
    }
}

fn update_creatures(
    creatures: &[[bool; CREATURE_SIZE_X]; CREATURE_SIZE_Y],
    new_array: &mut [[bool; CREATURE_SIZE_X]; CREATURE_SIZE_Y],
) {
    for x in 0..GRID_SIZE.x as i32 {
        for y in 0..GRID_SIZE.y as i32 {
            let neighbours = count_neighbour_cell(&creatures, x as usize, y as usize);
            let current_cell = creatures[x as usize][y as usize];

            if (current_cell && neighbours < 2) || (current_cell && neighbours > 3) {
                // creatures[x as usize][y as usize] = false;
                new_array[x as usize][y as usize] = false;
            } else if (current_cell && (neighbours == 2 || neighbours == 3))
                || (!current_cell && neighbours == 3)
            {
                // creatures[x as usize][y as usize] = true;
                new_array[x as usize][y as usize] = true;
            }
            //
            // if current_cell {
            //     println!("X: {} Y: {} NEIGHBOURS: {}", x, y, neighbours);
            // }
        }
    }
}

fn count_neighbour_cell(
    creatures: &[[bool; CREATURE_SIZE_X]; CREATURE_SIZE_Y],
    x: usize,
    y: usize,
) -> i32 {
    let mut neighbours = 0;

    if y != 0 && creatures[x][y - 1] {
        neighbours += 1;
    }
    if y != CREATURE_SIZE_Y - 1 && creatures[x][y + 1] {
        neighbours += 1;
    }
    if x != 0 && creatures[x - 1][y] {
        neighbours += 1;
    }
    if x != GRID_SIZE.x as usize - 1 && creatures[x + 1][y] {
        neighbours += 1;
    }
    if x != 0 && y != 0 && creatures[x - 1][y - 1] {
        neighbours += 1;
    }
    if x != GRID_SIZE.x as usize - 1 && y != CREATURE_SIZE_Y - 1 && creatures[x + 1][y + 1] {
        neighbours += 1;
    }
    if x != 0 && y != CREATURE_SIZE_Y - 1 && creatures[x - 1][y + 1] {
        neighbours += 1;
    }
    if x != GRID_SIZE.x as usize - 1 && y != 0 && creatures[x + 1][y - 1] {
        neighbours += 1;
    }

    neighbours
}

// fn count_neighbour_cell(
//     creatures: &[[bool; CREATURE_SIZE_X]; CREATURE_SIZE_Y],
//     x: usize,
//     y: usize,
// ) -> i32 {
//     let mut neighbours = 0;
//
//     if y != GRID_SIZE.y as usize - 1 && creatures[x][y + 1] {
//         neighbours += 1;
//     }
//     if y != 0 && y != GRID_SIZE.y as usize - 1 && creatures[x][y - 1] {
//         neighbours += 1;
//     }
//     if x != GRID_SIZE.x as usize - 1 && creatures[x + 1][y] {
//         neighbours += 1;
//     }
//     if x != GRID_SIZE.x as usize - 1 && y != GRID_SIZE.y as usize - 1 && creatures[x + 1][y + 1] {
//         neighbours += 1;
//     }
//     if y != 0
//         && y != GRID_SIZE.y as usize - 1
//         && x != GRID_SIZE.x as usize - 1
//         && creatures[x + 1][y - 1]
//     {
//         neighbours += 1;
//     }
//     if x != 0
//         && x != GRID_SIZE.x as usize
//         && y != 0
//         && y != GRID_SIZE.y as usize
//         && creatures[x - 1][y - 1]
//     {
//         neighbours += 1;
//     }
//     if x != 0
//         && x != GRID_SIZE.x as usize - 1
//         && y != GRID_SIZE.y as usize - 1
//         && creatures[x - 1][y + 1]
//     {
//         neighbours += 1;
//     }
//     if x != 0 && x != GRID_SIZE.x as usize - 1 && creatures[x - 1][y] {
//         neighbours += 1;
//     }
//
//     return neighbours;
// }

fn draw_creatures(
    d: &mut RaylibDrawHandle,
    creatures: &[[bool; CREATURE_SIZE_X]; CREATURE_SIZE_Y],
) {
    for x in 0..GRID_SIZE.x as i32 {
        for y in 0..GRID_SIZE.y as i32 {
            if creatures[x as usize][y as usize] {
                (*d).draw_rectangle(
                    x * CELL_SIZE.x as i32,
                    y * CELL_SIZE.y as i32,
                    CELL_SIZE.x as i32,
                    CELL_SIZE.y as i32,
                    Color::GREEN,
                );
            }
        }
    }
}

fn draw_grid(d: &mut RaylibDrawHandle) {
    for x in 0..GRID_SIZE.x as i32 {
        (*d).draw_line(
            x * CELL_SIZE.x as i32,
            0,
            x * CELL_SIZE.x as i32,
            WINDOW_SIZE.y as i32,
            Color::BLACK,
        );
    }

    for y in 0..GRID_SIZE.y as i32 {
        (*d).draw_line(
            0,
            y * CELL_SIZE.y as i32,
            WINDOW_SIZE.x as i32,
            y * CELL_SIZE.y as i32,
            Color::BLACK,
        );
    }
}
