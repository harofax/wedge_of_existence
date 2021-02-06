use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

mod map;
pub use map::*;

mod components;
pub use components::*;

mod player;
pub use player::*;

mod rect;
pub use rect::Rect;

mod configuration;
pub use configuration::*;

mod visibility_system;
use visibility_system::VisibilitySystem;



// --------- WORLD / GAMESTATE STUFF ----------------
pub struct State {
    ecs: World
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}


impl GameState for State {
    // tick = one frame i guess, one tick yknow
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}


fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
        .with_title("Wedge of Existence")
        .build()?;

    // create a game stat + add a new "world" to it (ecs lingo)
    let mut gs = State{
        ecs: World::new()
    };

    // register our components in the world: (can be anything that implements component!)
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let map : Map = Map::new_town_houses(MAX_WIDTH, MAX_HEIGHT); //Map::new_map_rooms_and_corridors(MAX_WIDTH, MAX_HEIGHT);

    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);


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
        .with(Viewshed { visible_tiles: Vec::new(), range: 20, dirty: true})
        .build();                                       // assembles the actual entity

    rltk::main_loop(context, gs)
}