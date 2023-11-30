use bevy::{prelude::*, utils::HashMap};

use crate::hex::coordinate::AxialCoordinate;

#[derive(Component, Hash, PartialEq, Eq)]
pub enum Piece {
  Pawn(PieceColor),
  Ranger(PieceColor),
  Flanker(PieceColor),
  Advancer(PieceColor),
  Queen(PieceColor),
  King(PieceColor),
}

impl Default for Piece {
  fn default() -> Self {
    Self::Pawn(Default::default())
  }
}

#[derive(Component, Default, Hash, PartialEq, Eq)]
pub enum PieceColor {
  #[default]
  White,
  Black,
}

#[derive(Component, Default, Hash, PartialEq, Eq)]
pub enum TileColor {
  #[default]
  White,
  Gray,
  Black,
}

// #[derive(Component, Default)]
// pub struct FileAndRank {
//   file: char,
//   rank: u32,
// }

#[derive(Hash, PartialEq, Eq)]
pub struct Tile {
  entity: Entity,
  piece: Piece,
}

#[derive(Resource, Default)]
pub struct Map(HashMap<AxialCoordinate, Tile>);