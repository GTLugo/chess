use bevy::prelude::*;

#[derive(Component, Default)]
pub enum PieceType {
  #[default]
  Pawn,
  Knight,
  Bishop,
  Rook,
  Queen,
  King,
}

#[derive(Component, Default)]
pub enum PieceColor {
  #[default]
  White,
  Black,
}

#[derive(Component, Default)]
pub enum TileColor {
  #[default]
  White,
  Gray,
  Black,
}