use bevy::sprite::{Sprite};
use bevy::transform::components::Transform;
use bevy::prelude::{Vec2,Vec3};
use crate::consts::TILE_SIZE;

#[derive(Default, Copy, Clone, PartialEq)]
pub struct TilePosition(pub Vec2);
impl TilePosition {
  pub fn get_translation(&self, size: Vec2) -> Vec3 {
    (self.0 * TILE_SIZE as f32 + size / 2.).extend(0.0)
  }
}

// position from top left of sprite
#[derive(Default, Copy, Clone, PartialEq)]
pub struct PixelPosition(pub Vec2);
impl PixelPosition {
  pub fn set_with_sprite_transform(s: Sprite, t: Transform) -> Self {
    PixelPosition(t.translation.truncate() - s.size / 2.)
  }

  // only allow integer positions of pixels
  pub fn get_translation(&self, size: Vec2) -> Vec3 {
    (self.0 + size / 2.).round().extend(0.0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use bevy::sprite::SpriteResizeMode;

  #[test]
  fn tile_position_get_translation() {
    let s = Sprite {
      size: Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
      resize_mode: SpriteResizeMode::default(),
    };

    let tile_pos = TilePosition(Vec2::new(22., 33.));
    let t = tile_pos.get_translation(s.size);

    assert_eq!(t, Vec3::new(22.5 * TILE_SIZE as f32, 33.5 * TILE_SIZE as f32, 0.0));
  }

  #[test]
  fn pixel_position_set_with_sprite_transform() {
    let s = Sprite {
      size: Vec2::new(1., 2.),
      resize_mode: SpriteResizeMode::default(),
    };

    let t = Transform::from_translation(Vec3::new(0., 0., 0.));

    let p = PixelPosition::set_with_sprite_transform(s, t);
    assert_eq!(p.0, Vec2::new(-0.5, -1.0));
  }

  #[test]
  fn pixel_position_get_transform() {
    let s = Sprite {
      size: Vec2::new(3., 2.),
      resize_mode: SpriteResizeMode::default(),
    };

    let p = PixelPosition(Vec2::new(10.0, 5.0));

    let t = p.get_translation(s.size);
    assert_eq!(t, Vec3::new(12.0, 6.0, 0.0));
  }
}

