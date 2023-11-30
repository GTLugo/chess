
use bevy::prelude::*;

pub enum Direction {
  NORTH,
  SOUTH,
  NORTHWEST,
  SOUTHWEST,
  NORTHEAST,
  SOUTHEAST,
}

#[derive(Component, Default, Hash, PartialEq, Eq)]
pub struct AxialCoordinate {
  q: i32,
  r: i32,
  // s: i32,
}

impl AxialCoordinate {
  pub const NORTH: Self     = Self { q:  0, r: -1, /*s:  1*/ };
  pub const SOUTH: Self     = Self { q:  0, r:  1, /*s: -1*/ };
  pub const NORTHWEST: Self = Self { q: -1, r:  0, /*s:  1*/ };
  pub const SOUTHWEST: Self = Self { q: -1, r:  1, /*s:  0*/ };
  pub const NORTHEAST: Self = Self { q:  1, r: -1, /*s:  0*/ };
  pub const SOUTHEAST: Self = Self { q:  1, r:  0, /*s: -1*/ };

  pub fn new(q: i32, r: i32) -> Self {
    Self { q, r, /*s: -(q + r)*/ }
  }

  pub fn q(&self) -> i32 {
    self.q
  }

  pub fn r(&self) -> i32 {
    self.r
  }

  pub fn s(&self) -> i32 {
    -(self.q + self.r)
  }

  pub fn length(&self) -> i32 {
    (self.q.abs() + self.r.abs() + self.s().abs()) / 2
  }

  pub fn distance_to(&self, rhs: Self) -> i32 {
    (self - rhs).length()
  }

  pub fn neighbor(&self, direction: Direction) -> Self {
    self + direction.into()
  }
}

impl std::ops::Add for AxialCoordinate {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self::Output {
      q: self.q + rhs.q,
      r: self.r + rhs.r,
      // s: self.s + rhs.s,
    }
  }
}

impl std::ops::Add<&AxialCoordinate> for AxialCoordinate {
  type Output = AxialCoordinate;

  fn add(self, rhs: &AxialCoordinate) -> Self::Output {
    Self::Output {
      q: self.q + rhs.q,
      r: self.r + rhs.r,
      // s: self.s + rhs.s,
    }
  }
}

impl std::ops::Add<AxialCoordinate> for &AxialCoordinate {
  type Output = AxialCoordinate;

  fn add(self, rhs: AxialCoordinate) -> Self::Output {
    Self::Output {
      q: self.q + rhs.q,
      r: self.r + rhs.r,
      // s: self.s + rhs.s,
    }
  }
}

impl std::ops::Sub for AxialCoordinate {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self::Output {
      q: self.q - rhs.q,
      r: self.r - rhs.r,
      // s: self.s - rhs.s,
    }
  }
}

impl std::ops::Sub<&AxialCoordinate> for AxialCoordinate {
  type Output = AxialCoordinate;

  fn sub(self, rhs: &AxialCoordinate) -> Self::Output {
    Self::Output {
      q: self.q - rhs.q,
      r: self.r - rhs.r,
      // s: self.s - rhs.s,
    }
  }
}

impl std::ops::Sub<AxialCoordinate> for &AxialCoordinate {
  type Output = AxialCoordinate;

  fn sub(self, rhs: AxialCoordinate) -> Self::Output {
    Self::Output {
      q: self.q - rhs.q,
      r: self.r - rhs.r,
      // s: self.s - rhs.s,
    }
  }
}

impl std::ops::Mul<i32> for AxialCoordinate {
  type Output = Self;

  fn mul(self, rhs: i32) -> Self::Output {
    Self::Output {
      q: self.q * rhs,
      r: self.r * rhs,
      // s: self.s * rhs,
    }
  }
}

impl std::ops::Mul<AxialCoordinate> for i32 {
  type Output = AxialCoordinate;

  fn mul(self, rhs: AxialCoordinate) -> Self::Output {
    Self::Output {
      q: rhs.q * self,
      r: rhs.r * self,
      // s: rhs.s * self,
    }
  }
}

impl From<Direction> for AxialCoordinate {
  fn from(value: Direction) -> Self {
    match value {
      Direction::NORTH     => Self::NORTH    ,
      Direction::SOUTH     => Self::SOUTH    ,
      Direction::NORTHWEST => Self::NORTHWEST,
      Direction::SOUTHWEST => Self::SOUTHWEST,
      Direction::NORTHEAST => Self::NORTHEAST,
      Direction::SOUTHEAST => Self::SOUTHEAST,
    }
  }
}