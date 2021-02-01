use rltk::{GameState, Rltk, RGB, BTermBuilder};
use specs::prelude::*;

mod map;
pub use map::*;

mod components;
pub use components::*;

mod player;
pub use player::*;

mod rect;
mod configuration;
pub use configuration::*;

pub use rect::Rect;

// --------- WORLD / GAMESTATE STUFF ----------------
pub struct State {
    ecs: World
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}


impl GameState for State {
    // tick = one frame i guess, one tick yknow
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

    let window = match RltkBuilder::simple(MAX_WIDTH, MAX_HEIGHT) {
        Ok(x) => {x},
        Err(e) => panic!(e)
    };

    let context = window
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

    let (rooms, map) = new_map_rooms_and_corridors(MAX_HEIGHT, MAX_WIDTH);

    gs.ecs.insert(map);
    let (player_x, player_y) = rooms[0].center();

    // time to add an entity!
    gs.ecs
        .create_entity()                     // creates empty entity
        .with(Position {x: player_x, y: player_y })   // adds a position component
        .with(Renderable {                           // adds a renderable component
            glyph: rltk::to_cp437('@'),              // cp437 == ascii sheet, check dwarf fortress wiki
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::from_u8(0,106,107),
        })
        .with(Player{})
        .build();                                       // assembles the actual entity

    rltk::main_loop(context, gs)
}