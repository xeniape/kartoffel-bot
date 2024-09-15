#![no_std]
#![no_main]

use kartoffel::*;

enum Direction {
    FRONT,
    LEFT,
    RIGHT,
}

#[no_mangle]
fn main() {
    loop {
        radar_wait();
        let scan = radar_scan_5x5();

        if let Some(enemy_position) = find_enemy(scan) {
            move_based_on_direction(convert_position_to_direction(enemy_position));
            if enemy_position == (1, 2) {
                arm_wait();
                arm_stab();
            }
        } else {
            if scan[1][2] == '.' {
                // Path is in front
                motor_wait();
                motor_step();
            } else if scan[2][1] == '.' {
                // Path is on left side
                motor_wait();
                motor_turn_left();
            } else if scan[2][3] == '.' {
                // Path is on right side
                motor_wait();
                motor_turn_right();
            }
        }
    }
}

fn find_enemy(map: [[char; 5]; 5]) -> Option<(usize, usize)> {
    let mut nearest_enemy: Option<(usize, usize)> = None;
    let mut best_distance: usize = 5;

    for i in 0..5 {
        for j in 0..5 {
            if i == 2 && j == 2 {
                continue;
            }
            if map[i][j] == '@' {
                let enemy_distance = get_distance_to_player((i, j));
                if enemy_distance < best_distance {
                    nearest_enemy = Some((i, j));
                    best_distance = enemy_distance;
                }
            }
        }
    }
    nearest_enemy
}

fn get_distance_to_player((i, j): (usize, usize)) -> usize {
    i.abs_diff(2) + j.abs_diff(2)
}

fn convert_position_to_direction((i, j): (usize, usize)) -> Direction {
    match (i, j) {
        (0, _) | (1, _) => Direction::FRONT,
        (_, 0) | (_, 1) => Direction::LEFT,
        (_, _) => Direction::RIGHT,
    }
}

fn move_based_on_direction(direction: Direction) {
    motor_wait();
    match direction {
        Direction::FRONT => {
            motor_step();
        }
        Direction::RIGHT => motor_turn_right(),
        Direction::LEFT => motor_turn_left(),
    }
}
