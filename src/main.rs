#![no_std]
#![no_main]

use kartoffel::*;

enum Direction {
    Front,
    Left,
    Right,
}

#[no_mangle]
fn main() {
    loop {
        arm_wait();
        radar_wait();
        if let Some(enemy_position) = find_enemy(radar_scan_3x3()) {
            move_based_on_direction(convert_position_to_direction(enemy_position, 3));
            if enemy_position == (1, 2) {
                arm_stab();
            }
        } else {
            radar_wait();
            let scan = radar_scan_5x5();
            if let Some(enemy_position) = find_enemy(scan) {
                move_based_on_direction(convert_position_to_direction(enemy_position, 5));
                if enemy_position == (1, 2) {
                    arm_stab();
                }
            } else if scan[1][2] == '.' {
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

fn find_enemy<const D: usize>(map: [[char; D]; D]) -> Option<(usize, usize)> {
    let mut nearest_enemy: Option<(usize, usize)> = None;
    let mut best_distance: usize = D;
    let player_coordinate = (D - 1) / 2;

    for i in 0..D {
        for j in 0..D {
            if i == player_coordinate && j == player_coordinate {
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

fn convert_position_to_direction((i, j): (usize, usize), scan_size: usize) -> Direction {
    match scan_size {
        3 => match (i, j) {
            (0, _) => Direction::Front,
            (_, 0) => Direction::Left,
            (_, _) => Direction::Right,
        },
        5 => match (i, j) {
            (0, _) | (1, _) => Direction::Front,
            (_, 0) | (_, 1) => Direction::Left,
            (_, _) => Direction::Right,
        },
        _ => Direction::Front,
    }
}

fn move_based_on_direction(direction: Direction) {
    motor_wait();
    match direction {
        Direction::Front => {
            motor_step();
        }
        Direction::Right => motor_turn_right(),
        Direction::Left => motor_turn_left(),
    }
}
