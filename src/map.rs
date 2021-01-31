use rltk::{ RGB, Rltk, RandomNumberGenerator};
use super::{Rect};
use std::cmp::{max, min};


// ---------- TILES ------------------------

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

// ------------- MAP ---------------------
// multiplies y pos by map width (80) and adds x.
// lets us get a tile by its x and why position, in
// the huge map tile vector.

/// gets a tile's index from its x and y position,
/// assuming the map-width is 80
pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

/// Makes a map with a boundary and 400 randomly placed wall-tiles.
/// For testing purposes only
pub fn new_map_test() -> Vec<TileType> {
    // map is just a huge vector with tiles, which is
    // why we need the xy_idx function to get the positions of tiles
    let mut map = vec![TileType::Floor; 80*50];

    for x in 0..80{
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }
    map
}

fn apply_room_to_map(room : &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1 ..= room.y2 {
        for x in room.x1 + 1 ..= room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

//fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32 ) {

//}

pub fn new_map_rooms_and_corridors() -> Vec<TileType> {
    let mut map = vec![TileType::Wall; 80*50];

    let room1 = Rect::new(20, 15, 10, 15);
    let room2 = Rect::new(35, 15, 10, 15);

    apply_room_to_map(&room1, &mut map);
    apply_room_to_map(&room2, &mut map);

    map
}


// Draws the map, one tile at a time
pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;

    for tile in map.iter() {
        // Render depending on tile type
        match tile {
            TileType::Floor => {
                ctx.set(x, y,  RGB::from_u8(0,156,160), RGB::from_u8(0,106,107), rltk::to_cp437('"'));
            }
            TileType::Wall => {
                ctx.set(x, y,  RGB::from_u8(5, 181, 102), RGB::from_u8(0,106,107), rltk::to_cp437('♣'));
            }
        }

        // Move coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

