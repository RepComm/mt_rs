
use bevy::prelude::V

struct Chunk {
  data: Vec<u8>,
  cubicWidth: u8
}

impl Chunk {
  pub fn new(cubicWidth: u8) -> Self {
    Self {
      data: vec![0; cubicWidth],
      cubicWidth,
    }
  }
  pub fn getPos (index: i32, out: Vec3) {

  }
}