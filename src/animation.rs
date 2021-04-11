// heavily borrowed from https://github.com/bevyengine/bevy/pull/1770/files

use bevy::asset::Handle;
use bevy::core::{Time, Timer};
use bevy::ecs::prelude::{Query, Res};
use bevy::sprite::{TextureAtlas, TextureAtlasSprite};
use std::time::Duration;

pub struct AnimationFrame {
    pub atlas_handle: Handle<TextureAtlas>,
    pub atlas_index: u32,
    pub duration: Duration,
}

pub struct Animation {
    pub frames: Vec<AnimationFrame>,
}

pub struct Animations {
    pub animations: Vec<Animation>,
}

#[derive(Default)]
pub struct Animator {
    pub current_animation: usize,
    pub last_animation: usize,
    pub current_frame: usize,
    pub timer: Timer,
}

pub fn sprite_animation_system(
    time: Res<Time>,
    mut query: Query<(
        &Animations,
        &mut Animator,
        &mut Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
    )>,
) {
    for (animations, mut animator, mut atlas, mut sprite) in &mut query.iter_mut() {
        animator.timer.tick(time.delta());

        if !animator.timer.finished() && animator.last_animation == animator.current_animation {
            break;
        }
        animator.last_animation = animator.current_animation;

        if let Some(animation) = animations.animations.get(animator.current_animation) {
            animator.current_frame = if animator.current_frame + 1 < animation.frames.len() {
                animator.current_frame + 1
            } else {
                0
            };

            if let Some(frame) = animation.frames.get(animator.current_frame) {
                animator.timer.set_duration(frame.duration);
                animator.timer.reset();
                *atlas = frame.atlas_handle.clone();
                sprite.index = frame.atlas_index;
            }
        }
    }
}
