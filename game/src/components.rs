use bevy::prelude::*;
use serde::Deserialize;

use crate::player_loop::Actions;

// region : general components

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}

#[derive(Component, Default)]
pub struct SpriteSizeAttack {
    pub size: Vec2,
    pub position: Vec3,
}

#[derive(Component)]
pub struct Attack {
    pub is_attacking: bool,
    pub is_attacked: bool,
}

#[derive(Component, Default, Clone)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
}

#[derive(Component, Default, Clone)]
pub struct Acceleration {
    pub ax: f32,
    pub ay: f32,
}

#[derive(Component)]
pub struct Grounded(pub bool);

#[derive(Component)]
pub struct RepeatAnimation(pub bool);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Deserialize)]
pub struct Platform {
    pub size: Vec2,
    pub position: Vec3,
}
#[derive(Component, Clone)]
pub struct MovingPlatform;
//endregion : general components

//region : player components

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct RecordingOn(pub bool);

#[derive(Component, Clone)]
pub struct Recording {
    pub index: usize,
    pub initial_pos: Vec3,
    pub initial_speed: Velocity,
    pub initial_acceleration: Acceleration,
    pub recorded_actions: Vec<Vec<Actions>>,
}

#[derive(Component)]
pub struct TemporalGhost;

//endregion : player components

//region : enemy components
#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Skeleton;

#[derive(Component)]
pub struct Eye;

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct FromEnemy;
//endregion : enemy components
