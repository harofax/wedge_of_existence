
use rltk::{RGB, Rltk, RandomNumberGenerator, Algorithm2D, Point, BaseMap};
use std::cmp::{max, min};
use super::{Rect};
use specs::prelude::*;
use std::ops::Mul;

// ---------- TILES ------------------------
#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor, Door
}

// ------------- MAP ---------------------
#[derive(Default)]
pub struct Map {
    pub tiles : Vec<TileType>,
    pub rooms : Vec<Rect>,
    pub width : i32,
    pub height : i32,
    pub revealed_tiles : Vec<bool>,
    pub visible_tiles : Vec<bool>
}

impl Map {
    /// gets a tile's index from its x and y position,
    /// assuming the map-width is 80
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 +1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_tiles_to_map(&mut self, rect: &Rect, tile: TileType) {
        for y in rect.y1 ..= rect.y2 {
            for x in rect.x1 ..= rect.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = tile;
            }
        }
    }

    fn make_door(&mut self, room: &Rect, dir: i8) {
        let mut door_y = 0;
        let mut door_x = 0;
        match dir {
            1 => { // top
                door_y = room.y1;
                door_x = (room.x1 + room.x2)/2;
            }
            2 => { // right
                door_x = room.x2;
                door_y = (room.y1 + room.y2)/2;
            }
            3 => { // bottom
                door_y = room.y2;
                door_x = (room.x1 + room.x2)/2;
            }
            4 => { // left
                door_x = room.x1;
                door_y = (room.y1 + room.y2)/2;
            }
            _ => {}
        }

        let door_idx = self.xy_idx(door_x, door_y);

        self.tiles[door_idx] = TileType::Door;
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1,x2) ..= max(x1,x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2) ..= max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    pub fn new_town_houses(width: usize, height: usize) -> Map {
        let mut map = Map{
            tiles : vec![TileType::Floor; width*height],
            rooms : Vec::new(),
            width : width as i32,
            height : height as i32,
            revealed_tiles : vec![false; width*height],
            visible_tiles : vec![false; width*height]
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _i in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;

            let new_room = Rect::new(x, y, w, h);
            let cavity = Rect::new(x + 1, y + 1, w - 2, h - 2);
            let mut ok = true;

            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) { ok = false }
            }

            if ok {

                map.apply_tiles_to_map(&new_room, TileType::Wall);
                map.apply_tiles_to_map(&cavity, TileType::Floor);
                map.make_door(&new_room, rng.roll_dice(1, 4) as i8);
                map.rooms.push(new_room);
            }
        }

        map
    }


    pub fn new_map_rooms_and_corridors(width: usize, height: usize) -> Map {
        let mut map = Map{
            tiles : vec![TileType::Wall; width*height],
            rooms : Vec::new(),
            width : width as i32,
            height : height as i32,
            revealed_tiles : vec![false; width*height],
            visible_tiles : vec![false; width*height]
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _i in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;

            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;

            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) { ok = false }
            }

            if ok {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();

                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel( prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel( prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel( prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel( prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx:usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        pos.x < self.width && pos.y < self.height && pos.x >= 0 && pos.y >= 0
    }
}

// Draws the map, one tile at a time
pub fn draw_map(ecs: &World, ctx : &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;
    for (idx,tile) in map.tiles.iter().enumerate() {

        if map.revealed_tiles[idx] {
            let glyph;
            let mut fg;
            let mut bg;
            match tile {
                TileType::Floor => {
                    fg = RGB::from_u8(2, 219, 158);
                    bg = RGB::from_u8(2, 168, 129);
                    glyph = rltk::to_cp437('"');
                }
                TileType::Wall => {
                    fg = RGB::from_u8(120,135,211);
                    bg = RGB::from_u8(62,60,137);
                    glyph = rltk::to_cp437('µ');
                }
                TileType::Door => {
                    fg = RGB::from_u8(255, 100, 100);
                    bg = RGB::from_u8(100, 50, 255);
                    glyph = rltk::to_cp437('O');
                }
            }
            if !map.visible_tiles[idx] {
                fg = fg.to_greyscale().mul(0.5);
                bg = bg.to_greyscale().mul(0.5);
            }
            ctx.set(x, y, fg, bg, glyph);
        }

        x += 1;
        if x > map.width - 1 {
            x = 0;
            y += 1;
        }
    }
}


