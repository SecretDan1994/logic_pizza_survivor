use crate::{
    attack::{spawn_area_shot, spawn_close_shot, spawn_whip},
    prelude::*,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player).add_systems(
            (
                player_movement,
                player_exp_start_pickup,
                player_gain_exp,
                player_level_up,
            )
                .in_set(OnUpdate(GameState::Gameplay)),
        );
    }
}

fn player_level_up(mut player: Query<&mut Player>, mut game_state: ResMut<NextState<GameState>>) {
    let mut player = player.single_mut();

    if player.exp >= player.next_level_exp {
        player.exp = 0;
        player.next_level_exp = (player.next_level_exp as f32 * 1.4) as i64;
        player.level += 1;
        game_state.set(GameState::LevelUp);
    }
}

fn player_exp_start_pickup(
    player: Query<(&Transform, &Collider), With<Player>>,
    rapier_context: Res<RapierContext>,
    mut orbs: Query<&mut ExpOrb>,
) {
    let (transform, collider) = player.single();

    rapier_context.intersections_with_shape(
        transform.translation.truncate(),
        0.0,
        collider,
        QueryFilter::new(),
        |entity| {
            if let Ok(mut orb) = orbs.get_mut(entity) {
                orb.collecting = true;
            }
            true
        },
    );
}

fn player_gain_exp(
    mut commands: Commands,
    orbs: Query<(Entity, &Transform, &ExpOrb)>,
    mut player: Query<(&Transform, &mut Player), Without<ExpOrb>>,
) {
    let (player_transform, mut player) = player.single_mut();

    for (entity, transform, orb) in &orbs {
        //TODO probably should use physics for this
        if Vec2::distance(
            transform.translation.truncate(),
            player_transform.translation.truncate(),
        ) < 0.3
        {
            //TODO event for sound
            player.exp += orb.value;
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_player(mut commands: Commands) {
    let whip = spawn_whip(&mut commands);
    let close = spawn_close_shot(&mut commands);
    let area = spawn_area_shot(&mut commands);
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 100.0),
                ..default()
            },
            Player {
                exp: 0,
                next_level_exp: 5,
                level: 1,
                speed: 3.0,
                health: 100.0,
                max_health: 100.0,
                facing: Facing::Left,
            },
            Name::new("Player"),
            Collider::ball(0.7),
        ))
        .add_child(whip)
        .add_child(close)
        .add_child(area);
}

fn player_movement(
    mut player: Query<(&mut Transform, &mut Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut player) = player.single_mut();
    if input.pressed(KeyCode::W) {
        transform.translation.y += time.delta_seconds() * player.speed;
    }
    if input.pressed(KeyCode::S) {
        transform.translation.y -= time.delta_seconds() * player.speed;
    }
    if input.pressed(KeyCode::A) {
        transform.translation.x -= time.delta_seconds() * player.speed;
        player.facing = Facing::Left;
    }
    if input.pressed(KeyCode::D) {
        transform.translation.x += time.delta_seconds() * player.speed;
        player.facing = Facing::Right;
    }
}
