use crate::api::sokoban_service::{Sokoban};
use crate::api::constants::{BOX_ON_TARGET_U8, BOX_U8, DOWN, EMPTY_PLACE_U8, LEFT, PLAYER_U8, TARGET_U8, UP, WALL_U8};
use crate::api::coord_service::{Coord, update_coords};
use crate::api::map_service::refresh_map;
use crate::api::ux::print_map;

#[derive(Debug)]
pub enum Move {
    Up,
    Left,
    Down,
    Right,
}

pub fn process_input(input: &str) -> Move {
    return if input == UP {
        Move::Up
    } else if input == LEFT {
        Move::Left
    } else if input == DOWN {
        Move::Down
    } else {
        Move::Right
    };
}

pub fn get_deltas(movement: Move) -> (i8, i8) {
    let mut delta_x: i8 = 0;
    let mut delta_y: i8 = 0;

    match movement {
        Move::Up => delta_y = -1,
        Move::Left => delta_x = -1,
        Move::Down => delta_y = 1,
        Move::Right => delta_x = 1,
    };
    (delta_x, delta_y)
}

pub fn get_next_valid_coord(
    user_coords: &Coord,
    delta_x: i8,
    delta_y: i8,
    rows: &usize,
    columns: &usize,
) -> Coord {
    let mut new_coord_x = user_coords.x as i8 + delta_x;
    let mut new_coord_y = user_coords.y as i8 + delta_y;

    if (new_coord_x >= *columns as i8) || (new_coord_x < 0) {
        new_coord_x = user_coords.x as i8;
    };
    if (new_coord_y >= *rows as i8) || (new_coord_y < 0) {
        new_coord_y = user_coords.y as i8;
    };
    Coord {
        x: new_coord_x as usize,
        y: new_coord_y as usize,
    }
}

pub fn is_object(next_coord: &Coord, object_to_compare: u8, map: &Vec<Vec<u8>>) -> bool {
    return if map[next_coord.y as usize][next_coord.x as usize] == object_to_compare {
        true
    } else {
        false
    };
}

pub fn process_move(sokoban: &mut Sokoban, movement: Move) {
    let (delta_x, delta_y) = get_deltas(movement);
    let mut next_coord: Coord = get_next_valid_coord(&sokoban.user_coords, delta_x, delta_y, &sokoban.rows, &sokoban.columns);
    let mut next_next_coord = get_next_valid_coord(&next_coord, delta_x, delta_y, &sokoban.rows, &sokoban.columns);

    if is_object(&next_coord, WALL_U8, &sokoban.map) {
        return;
    }

    if is_object(&next_coord, BOX_U8, &sokoban.map)
    || is_object(&next_coord, BOX_ON_TARGET_U8, &sokoban.map) {

        if is_object(&next_next_coord, WALL_U8, &sokoban.map)
            || is_object(&next_next_coord, BOX_U8, &sokoban.map)
            || is_object(&next_next_coord, BOX_ON_TARGET_U8, &sokoban.map)
        {
            return;
        }
        move_box(&mut sokoban.map, &mut next_coord, &mut next_next_coord, &sokoban.target_coords, &mut sokoban.boxes_on_target_coords, &mut sokoban.boxes_coords);
    }

    move_player(&mut sokoban.map, &mut sokoban.user_coords, &next_coord, &sokoban.target_coords, &sokoban.boxes_on_target_coords, &sokoban.boxes_coords);
}

fn move_player(map: &mut Vec<Vec<u8>>, coords_from: &mut Coord, coords_to: &Coord, target_coords: &Vec<Coord>,
               boxes_on_target_coords: &Vec<Coord>, boxes_coords: &Vec<Coord>) {
    refresh_map(map, coords_from, coords_to, target_coords, PLAYER_U8, boxes_on_target_coords, boxes_coords);
    update_coords(coords_from, coords_to);
}

fn move_box(
    map: &mut Vec<Vec<u8>>,
    coords_from: &mut Coord,
    coords_to: &mut Coord,
    target_coords: &Vec<Coord>,
    boxes_on_target_coords: &mut Vec<Coord>,
    boxes_coords: &mut Vec<Coord>,
) {
    let move_to_target = is_object(&coords_to, TARGET_U8, map); // meto box en target
    let move_from_target = is_object(&coords_from, BOX_ON_TARGET_U8, map); // empujo box de target
    match boxes_coords.iter().position(|b| (b.y == coords_from.y) && (b.x == coords_from.x)){
        None => {}
        Some(index_to_remove) => {
            boxes_coords.remove(index_to_remove);
            boxes_coords.push(Coord{x:coords_to.x, y:coords_to.y});
        }
    }
    refresh_map(map, coords_from, coords_to, target_coords, BOX_U8, boxes_on_target_coords, boxes_coords);

    if move_to_target {
        push_target(coords_to, boxes_on_target_coords, map);
    }

    if move_from_target{
        pop_target(coords_from, coords_to, boxes_on_target_coords, map);
    }
}

fn pop_target(coords_from: &mut Coord, coords_to: &mut Coord, boxes_on_target_coords: &mut Vec<Coord>, map: &mut Vec<Vec<u8>>){
    match boxes_on_target_coords.iter().position(|b| (b.y == coords_from.y) && (b.x == coords_from.x)){
        None => {}
        Some(index_to_remove) => {
            boxes_on_target_coords.remove(index_to_remove);
            map[coords_to.y ][coords_to.x] = BOX_U8;
        }
    }

}

fn push_target(next_next_coord: &Coord, boxes_on_target_coords: &mut Vec<Coord>, map: &mut Vec<Vec<u8>>){
    let coord = Coord{x: next_next_coord.x, y:next_next_coord.y};
    boxes_on_target_coords.push(coord);
    map[next_next_coord.y ][next_next_coord.x] = BOX_ON_TARGET_U8;
}

pub fn won_game(sokoban: &mut Sokoban) -> bool {
    return if sokoban.boxes_on_target_coords.len() == sokoban.target_coords.len() {
        print_map(sokoban);
        true
    } else { false }
}