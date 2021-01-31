use specs::prelude::*;
use specs_derive::*;
use rltk::{RGB};

// -------------- COMPONENTS -------------------

// #[derive(Component)] below
//           ==
// impl Component for Position {
//    type Storage = VecStorage<Self>;
// }

// Position component, for stuff that needs places
#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

// Stuff that needs to be drawn
#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Debug)]
pub struct Player {}