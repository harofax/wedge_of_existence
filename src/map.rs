use rltk::{ RGB, Rltk, RandomNumberGenerator};
use super::{Rect,MAX_WIDTH, MAX_HEIGHT};
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
    (y as usize * MAX_WIDTH) + x as usize
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
    for y in room.y1 +1 ..= room.y2 {
        for x in room.x1 + 1 ..= room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32 ) {
    for x in min(x1, x2) ..= max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < MAX_WIDTH*MAX_HEIGHT {
            map[idx as usize] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32 ) {
    for y in min(y1, y2) ..= max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < MAX_WIDTH*MAX_HEIGHT {
            map[idx as usize] = TileType::Floor;
        }
    }
}

pub fn new_map_rooms_and_corridors(width: usize, height: usize) -> (Vec<Rect>, Vec<TileType>) {
    let mut map = vec![TileType::Wall; width*height];

    let mut rooms : Vec<Rect> = Vec::new();
    const MAX_ROOMS : i32 = 30;
    const MIN_SIZE : i32 = 6;
    const MAX_SIZE : i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, MAX_WIDTH as i32- w - 1)  - 1;
        let y = rng.roll_dice(1, MAX_HEIGHT as i32 - h - 1) - 1;

        let new_room = Rect::new(x, y, w, h);
        let mut ok = true;

        for other_room in rooms.iter() {
            if new_room.intersect(other_room) { ok = false }
        }

        if ok {
            apply_room_to_map(&new_room, &mut map);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len()-1].center();

                if rng.range(0, 2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                } else {
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                }
            }

            rooms.push(new_room);
        }
    }

    (rooms, map)
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
        if x > MAX_WIDTH - 1 {
            x = 0;
            y += 1;
        }
    }
}

