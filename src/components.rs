pub use crate::prelude::*;

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Render {
    pub color: ColorPair, // helper class from bracket-lib- stores both forground and background color in a single struct
    pub glyph: FontCharType // defined in bracket-lib to store a single character or glyph
}

#[derive(Clone,Copy,Debug,PartialEq)]
// tag - indicating that an entity with this component is the player
pub struct Player {

}