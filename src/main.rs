use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;

// ---------- TILES ------------------------

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall, Floor
}

// ------------- MAP ---------------------
// multiplies y pos by map width (80) and adds x.
// so it basically gives you an x, y position in a
// vector, that corresponds to x and y. y * map_width + x!
// * * * * * * = map_width * y (for several rows
// +
// * * * = x
pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

fn new_map() -> Vec<TileType> {
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

fn draw_map(map: &[TileType], ctx: &mut Rltk) {
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


// -------------- COMPONENTS -------------------

// #[derive(Component)] below same as:
// impl Component for Position {
//    type Storage = VecStorage<Self>;
// }
#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component, Debug)]
struct Player {}

// ------- SYSTEMS ---------------


// ------------ WORLD and GameStates ----------------
struct State {
    ecs: World
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);

        if map[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        let map = self.ecs.fetch::<Vec<TileType>>();

        draw_map(&map, ctx);

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }


    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
        .with_title("Wedge of Life")
        .build()?;

    // create a game stat + add a new "world" to it (ecs lingo)
    let mut gs = State{
        ecs: World::new()
    };

    // register our components in the world: (can be anything that implements component!)
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    gs.ecs.insert(new_map());

    // time to add an entity!
    gs.ecs
        .create_entity()                     // creates empty entity
        .with(Position {x: 40, y: 25 })   // adds a position component
        .with(Renderable {                           // adds a renderable component
            glyph: rltk::to_cp437('@'),              // cp437 == ascii sheet, check dwarf fortress wiki
            fg: RGB::named(rltk::RED1),
            bg: RGB::from_u8(0,106,107),
        })
        .with(Player{})
        .build();                                       // assembles the actual entity

    rltk::main_loop(context, gs)
}