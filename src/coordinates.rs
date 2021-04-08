use crate::consts::{AppState, TILE_SIZE};
use bevy::prelude::*;
use bevy::sprite::Sprite;
use bevy::transform::components::Transform;
use serde::{Deserialize, Serialize};

#[derive(Default, Copy, Clone, PartialEq)]
pub struct Velocity(pub Vec2);
pub struct Acceleration(pub Vec2);

pub struct SpriteSize(pub Vec2);
pub struct Layer(pub f32);

#[derive(Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct TilePosition(pub Vec2);
impl TilePosition {
    pub fn get_translation(&self, size: Vec2, z: f32) -> Vec3 {
        (self.0 * TILE_SIZE as f32 + size / 2.).extend(z)
    }

    pub fn get_pixel_position(&self) -> PixelPosition {
        PixelPosition(self.0 * TILE_SIZE as f32)
    }
}

// position from top left of sprite
#[derive(Default, Copy, Clone, PartialEq)]
pub struct PixelPosition(pub Vec2);
impl PixelPosition {
    // pub fn set_with_sprite_transform(s: Sprite, t: Transform) -> Self {
    //     PixelPosition(t.translation.truncate() - s.size / 2.)
    // }

    // only allow integer positions of pixels
    pub fn get_translation(&self, size: Vec2, z: f32) -> Vec3 {
        // round positions to keep pixels on grid
        // TODO: round is probably incorrect if sprite has odd (1, 3, 5, ...) dimensions
        (self.0 + size / 2.).round().extend(z)
    }
}

fn update_velocity(mut q: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (a, mut v) in q.iter_mut() {
        v.0 += a.0 * time.delta_seconds();
    }
}

fn update_position(mut q: Query<(&Velocity, &mut PixelPosition)>, time: Res<Time>) {
    for (v, mut p) in q.iter_mut() {
        p.0 += v.0 * time.delta_seconds();
    }
}

// TODO: add Changed<PixelPosition> here after upgrading to 5.0
// TODO: figure out how to unify update_translation and update_translation_atlas_sprite
fn update_translation(mut q: Query<(&PixelPosition, &Sprite, &mut Transform, &Layer)>) {
    for (pos, sprite, mut transform, layer) in q.iter_mut() {
        transform.translation = pos.get_translation(sprite.size, layer.0);
    }
}

fn update_translation_atlas_sprite(mut q: Query<(&PixelPosition, &SpriteSize, &mut Transform, &Layer)>) {
    for (pos, size, mut transform, layer) in q.iter_mut() {
        transform.translation = pos.get_translation(size.0, layer.0);
    }
}

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_update(AppState::InGame).with_system(update_velocity.system()).with_system(update_position.system()).with_system(update_translation.system()).with_system(update_translation_atlas_sprite.system()));
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
            ..Default::default()
        };

        let tile_pos = TilePosition(Vec2::new(22., 33.));
        let t = tile_pos.get_translation(s.size, 0.0);

        assert_eq!(
            t,
            Vec3::new(22.5 * TILE_SIZE as f32, 33.5 * TILE_SIZE as f32, 0.0)
        );
    }

    #[test]
    // fn pixel_position_set_with_sprite_transform() {
    //     let s = Sprite {
    //         size: Vec2::new(1., 2.),
    //         resize_mode: SpriteResizeMode::default(),
    //     };

    //     let t = Transform::from_translation(Vec3::new(0., 0., 0.));

    //     let p = PixelPosition::set_with_sprite_transform(s, t);
    //     assert_eq!(p.0, Vec2::new(-0.5, -1.0));
    // }
    #[test]
    fn pixel_position_get_transform() {
        let s = Sprite {
            size: Vec2::new(3., 2.),
            resize_mode: SpriteResizeMode::default(),
            ..Default::default()
        };

        let p = PixelPosition(Vec2::new(10.0, 5.0));

        let t = p.get_translation(s.size, 0.0);
        assert_eq!(t, Vec3::new(12.0, 6.0, 0.0));
    }
}
