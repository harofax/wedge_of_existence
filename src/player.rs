use rltk::{VirtualKeyCode, Rltk};
use specs::prelude::*;
use super::{Position, Player, TileType, Viewshed, State, Map};
use std::cmp::{min, max};

pub fn cheat_reveal_map(ecs: &mut World){
    let mut map = ecs.fetch_mut::<Map>();
    for tile in map.revealed_tiles.iter_mut() { *tile = true;}
}

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    // Fetch all entities that have a player and position component
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewshed = ecs.write_storage::<Viewshed>();

    // Fetch the map
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewshed).join() {
        let dest_x = pos.x + delta_x;
        let dest_y = pos.y + delta_y;

        if dest_x < 0 || dest_x > map.width - 1 || dest_y < 0 || dest_y > map.height - 1 {
            break;
        }

        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min((map.width - 1) as i32, max(0, pos.x + delta_x));
            pos.y = min((map.height - 1) as i32, max(0, pos.y + delta_y));

            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 => try_move_player(-1, 0, &mut gs.ecs),

            VirtualKeyCode::Right |
            VirtualKeyCode::Numpad6 => try_move_player(1, 0, &mut gs.ecs),

            VirtualKeyCode::Up |
            VirtualKeyCode::Numpad8 => try_move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 => try_move_player(0, 1, &mut gs.ecs),

            VirtualKeyCode::A => cheat_reveal_map(&mut gs.ecs),

            _ => {}
        },
    }
}