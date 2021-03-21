use crate::consts::{AppState, APP_STATE_STAGE};
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

#[derive(Copy, Clone, PartialEq)]
pub struct Box {
    pub offset: Vec2, // position relative to parent
    pub size: Vec2,
}
impl Default for Box {
    fn default() -> Self {
        Box {
            offset: Vec2::new(0.0, 0.0),
            size: Vec2::new(1.0, 1.0),
        }
    }
}

pub struct Hurtbox(Box);
impl Hurtbox {
    pub fn new(offset: Vec2, size: Vec2) -> Self {
        Hurtbox(Box { offset, size })
    }
}
impl std::ops::Deref for Hurtbox {
    type Target = Box;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Hitbox(Box);
impl Hitbox {
    pub fn new(offset: Vec2, size: Vec2) -> Self {
        Hitbox(Box { offset, size })
    }
}
impl std::ops::Deref for Hitbox {
    type Target = Box;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// systems detect collision between Hurtbox and Hitbox,
// emit event that should be handled?
// game has 3 types of collisions
// 1. player collides with wall
// 2. player collides with car
// 3. player collides with goal

fn collision_system(
    hitboxes: Query<(&Hitbox, &Transform)>,
    hurtboxes: Query<(&Hurtbox, &Transform)>,
) {
    for (hurtbox, hurt_transform) in hurtboxes.iter() {
        let hurt_top_left = hurt_transform.translation + hurtbox.offset.extend(0.0);
        let hurt_size = hurtbox.size;

        for (hitbox, hit_transform) in hitboxes.iter() {
            let hit_top_left = hit_transform.translation + hitbox.offset.extend(0.0);
            let hit_size = hitbox.size;

            if let Some(collision) = collide(hurt_top_left, hurt_size, hit_top_left, hit_size) {
                dbg!("collision");
            }
        }
    }
}

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_update(APP_STATE_STAGE, AppState::InGame, collision_system.system());
    }
}
