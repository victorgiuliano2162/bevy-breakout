#[allow(unused, dead_code)]


use bevy::math::vec3;
use bevy::prelude::*;

const PADDLE_START_Y: f32 = 0.0;
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PADDLE_SPEED: f32 = 500.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb( 0.9, 0.9, 0.9)))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_paddle)
        .run();
}

#[derive(Component)]
struct Paddle;

fn setup(mut commands: Commands) {
    //camera
    commands.spawn(Camera2dBundle::default());

    //paddle
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: vec3(0., PADDLE_START_Y, 0.),
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                custom_size: Some(PADDLE_SIZE),
                ..default()
            },
            ..default()
        },
        Paddle,
    ));
}

fn move_paddle(
    input: Res<Input<KeyCode>>,
    time_step: Res<FixedTime>,
    mut query: Query<&mut Transform, With<Paddle>>
) {
    let mut paddle_transform = query.single_mut();
    let mut direction_x = 0.0;

    if input.pressed(KeyCode::A) {
        direction_x -= 1.0;
    }

    if input.pressed(KeyCode::D) {
        direction_x += 1.0;
    }

    let new_x = paddle_transform.translation.x + direction_x * PADDLE_SPEED * time_step.period.as_secs_f32();

    paddle_transform.translation.x = new_x;

    //y vertice
    let mut direction_y = 0.0;

    if input.pressed(KeyCode::S) {
        direction_y -= 1.0;
    }

    if input.pressed(KeyCode::W) {
        direction_y += 1.0;
    }

    let new_y = paddle_transform.translation.y + direction_y * PADDLE_SPEED * time_step.period.as_secs_f32();

    paddle_transform.translation.y = new_y;
}
