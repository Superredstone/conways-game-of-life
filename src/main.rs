use raylib::prelude::*;

const WINDOW_SIZE: Vector2 = Vector2::new(800.0, 800.0);
const GRID_SIZE: Vector2 = Vector2::new(50.0, 50.0);
const CREATURE_SIZE_X: usize = GRID_SIZE.x as usize;
const CREATURE_SIZE_Y: usize = GRID_SIZE.y as usize;
const CELL_SIZE: Vector2 = Vector2::new(WINDOW_SIZE.x / GRID_SIZE.x, WINDOW_SIZE.y / GRID_SIZE.y);

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

    let mut do_next_frame = false;
    let mut frame_number = 0;

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_D) {
            do_next_frame = true;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        // --- Draw ---
        draw_creatures(&mut d, &creatures);
        draw_grid(&mut d);

        d.draw_text(
            &format!("Life cycles: {}", frame_number),
            10,
            10,
            30,
            Color::BLACK,
        );

        if !do_next_frame {
            continue;
        }

        // --- Update ---
        update_creatures(&mut creatures);

        frame_number += 1;
        do_next_frame = false;
    }
}

fn update_creatures(creatures: &mut [[bool; CREATURE_SIZE_X]; CREATURE_SIZE_Y]) {
    for x in 0..GRID_SIZE.x as i32 {
        for y in 0..GRID_SIZE.y as i32 {
            let neighbours = count_neighbour_cell(&creatures, x as usize, y as usize);
            let current_cell = creatures[x as usize][y as usize];

            if (current_cell && neighbours < 2) || (current_cell && neighbours > 3) {
                creatures[x as usize][y as usize] = false;
            } else if (current_cell && (neighbours == 2 || neighbours == 3))
                || (!current_cell && neighbours == 3)
            {
                creatures[x as usize][y as usize] = true;
            }

            if current_cell {
                println!("X: {} Y: {} NEIGHBOURS: {}", x, y, neighbours);
            }
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

    return neighbours;
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
