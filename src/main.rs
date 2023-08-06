use std::time::Duration;

use bevy::{input::common_conditions::input_toggle_active, time::Stopwatch};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use pizza_survivor::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pizza Survivor".into(),
                        resolution: (WIDTH, HEIGHT).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_plugin(RngPlugin::default())
        //.add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(WaveManager {
            global_time: Stopwatch::new(),
            waves: vec![
                Wave {
                    next_spawn: Timer::from_seconds(1.4, TimerMode::Repeating),
                    wave_size: 2,
                    to_spawn: Enemy {
                        speed: 1.3,
                        health: 5.0,
                        asset: "student_1.png".to_string(),
                        damage_per_second: 10.0,
                    },
                },
                Wave {
                    next_spawn: Timer::from_seconds(0.5, TimerMode::Repeating),
                    wave_size: 2,
                    to_spawn: Enemy {
                        speed: 2.2,
                        health: 1.0,
                        asset: "sorority_2.png".to_string(),
                        damage_per_second: 3.0,
                    },
                },
                Wave {
                    next_spawn: Timer::from_seconds(10.0, TimerMode::Repeating),
                    wave_size: 10,
                    to_spawn: Enemy {
                        speed: 0.8,
                        health: 30.0,
                        asset: "sorority_1.png".to_string(),
                        damage_per_second: 10.0,
                    },
                },
                Wave {
                    next_spawn: Timer::from_seconds(5.0, TimerMode::Repeating),
                    wave_size: 8,
                    to_spawn: Enemy {
                        speed: 2.3,
                        health: 5.0,
                        asset: "student_2.png".to_string(),
                        damage_per_second: 1.0,
                    },
                },
            ],
        })
        .add_state::<GameState>()
        .add_plugin(UpgradePlugin)
        .add_plugin(ExpPlugin)
        .add_plugin(GameCameraPlugin)
        .add_plugin(AttackPlugin)
        .add_plugin(GameUiPlugin)
        .add_plugin(GameAnimationPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_startup_system(spawn_background)
        .add_system(advance_state.in_set(OnUpdate(GameState::StartingLoop)))
        .add_system(despawn_game_play.in_schedule(OnEnter(GameState::GameOver)))
        .add_startup_system(spawn_coin_assets)
        .add_startup_system(spawn_magnet)
        .add_startup_system(start_music)
        .run();
}
fn start_music(audio: Res<Audio>, assets: Res<AssetServer>) {
    audio.play_with_settings(
        assets.load("background.mp3"),
        PlaybackSettings {
            repeat: true,
            volume: 0.25,
            speed: 1.0,
        },
    );
}

fn spawn_coin_assets(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(CoinAssets {
        image_1: assets.load("coin_1.png"),
        image_2: assets.load("coin_2.png"),
        audio: assets.load("coin.wav"),
    })
}

fn spawn_magnet(mut commands: Commands, assets: Res<AssetServer>){
    commands.insert_resource(MagnetAssets {
        image_1: assets.load("magnet.png"),
    })
}

// Just to prevent on enter gameplay getting called every time after level up
fn advance_state(mut state: ResMut<NextState<GameState>>, mut game_manager: ResMut<WaveManager>) {
    game_manager.global_time.set_elapsed(Duration::from_secs(0));
    state.set(GameState::Gameplay);
}

fn despawn_game_play(mut commands: Commands, entities: Query<Entity, With<GamePlayEntity>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_background(mut commands: Commands, assets: Res<AssetServer>) {
    let size = 1080.0 * PIXEL_TO_WORLD;
    for i in -7..7 {
        for j in -7..7 {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(i as f32 * size, j as f32 * size, 0.0),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(size, size)),
                        ..default()
                    },
                    texture: assets.load("background.png"),
                    ..default()
                },
                Name::new("Background"),
            ));
        }
    }
}
