use crate::car::Car;
use crate::consts::{AppState, SystemLabels};
use crate::map::Wall;
use crate::player::Player;
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use std::marker::PhantomData;

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

#[derive(Component)]
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

#[derive(Component)]
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
pub struct CollisionData {
    collided_with: Entity,
    collision: Collision,
}
pub struct CollisionEvent<S, T>(CollisionData, PhantomData<S>, PhantomData<T>);
impl<S, T> CollisionEvent<S, T> {
    fn new(data: CollisionData) -> Self {
        Self(data, PhantomData, PhantomData)
    }
}

fn collision_system(
    hurtboxes: Query<(&Hurtbox, &Transform, Option<&Player>)>,
    hitboxes: Query<(&Hitbox, &Transform, Entity, Option<&Car>, Option<&Wall>)>,
    mut ev_player_hitby_car: EventWriter<CollisionEvent<Player, Car>>,
    mut ev_player_hitby_wall: EventWriter<CollisionEvent<Player, Wall>>,
) {
    for (hurtbox, hurt_transform, player) in hurtboxes.iter() {
        let hurt_top_left = hurt_transform.translation + hurtbox.offset.extend(0.0);
        let hurt_size = hurtbox.size;

        for (hitbox, hit_transform, hit_entity, car, wall) in hitboxes.iter() {
            let hit_top_left = hit_transform.translation + hitbox.offset.extend(0.0);
            let hit_size = hitbox.size;

            if let Some(collision) = collide(hurt_top_left, hurt_size, hit_top_left, hit_size) {
                if player.is_some() && car.is_some() {
                    ev_player_hitby_car.send(CollisionEvent::new(CollisionData {
                        collided_with: hit_entity,
                        collision,
                    }));
                } else if player.is_some() && wall.is_some() {
                    ev_player_hitby_wall.send(CollisionEvent::new(CollisionData {
                        collided_with: hit_entity,
                        collision,
                    }));
                }
            }
        }
    }
}

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent<Player, Car>>()
            .add_event::<CollisionEvent<Player, Wall>>()
            .add_system_set(
                SystemSet::on_update(AppState::InGame).with_system(
                    collision_system
                        .after("update_translation")
                        .before(SystemLabels::PlayerMovement),
                ),
            );
    }
}
