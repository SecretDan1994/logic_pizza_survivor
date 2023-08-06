use crate::prelude::*;

impl Default for MagnetBundle {
    fn default() -> Self {
        Self {
            sprite: SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 100.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(0.4, 0.4)),
                    ..default()
                },
                ..default()
            },
            magnet: Magnet {
                active: false,
            },
            game_play: GamePlayEntity,
            collider: Collider::ball(1.0),
            sensor: Sensor,
        }
    }
}