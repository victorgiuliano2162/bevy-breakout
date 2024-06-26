#[allow(unused, dead_code)]
use bevy::math::vec3;
use bevy::{math::vec2, prelude::*, text::DEFAULT_FONT_HANDLE};

//paddle
const PADDLE_START_Y: f32 = 0.0;
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PADDLE_SPEED: f32 = 500.0;

//ball
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const BALL_SPEED: f32 = 400.0;
const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

//wall
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const WALL_THICKNESS: f32 = 10.;
const WALL_BLOCK_WIDTH: f32 = RIGHT_WALL - LEFT_WALL;
const WALL_BLOCK_HEIGHT: f32 = TOP_WALL - BOTTOM_WALL;
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (move_paddle, apply_velocity))
        .run();
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider {
    size: Vec2,
}

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    //ball
    let ball_text = asset_server.load("textures/circle.png");
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: BALL_STARTING_POSITION,
                ..default()
            },
            sprite: Sprite {
                color: BALL_COLOR,
                custom_size: Some(BALL_SIZE),
                ..default()
            },
            texture: ball_text,
            ..default()
        },
        Ball,
        Velocity(BALL_SPEED * BALL_INITIAL_DIRECTION), //Velocity(Vec2::ZERO)
    ));

    //walls
    {
        let vertical_wall_size = vec2(WALL_THICKNESS, WALL_BLOCK_HEIGHT + WALL_THICKNESS);
        let horizontal_wall_size = vec2(WALL_BLOCK_WIDTH + WALL_THICKNESS, WALL_THICKNESS);

        //left wall
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: vec3(LEFT_WALL, 0.0, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(vertical_wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider {
                size: horizontal_wall_size,
            },
        });

        //top wall
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: vec3(0.0, TOP_WALL, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(horizontal_wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider {
                size: horizontal_wall_size,
            },
        });
        
        //right wall 
        {
            commands.spawn(WallBundle {
                sprite_bundle: SpriteBundle {
                    transform: Transform {
                        translation: vec3(RIGHT_WALL, 0.0, 0.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color: WALL_COLOR,
                        custom_size: Some(vertical_wall_size),
                        ..default()
                    },
                    ..default()
                },
                collider: Collider {
                    size: vertical_wall_size,
                },
            });
        }

        //bottom wall
        {
            commands.spawn( WallBundle {
                sprite_bundle: SpriteBundle {
                    transform: Transform {
                    translation: vec3(0.0, BOTTOM_WALL, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(horizontal_wall_size),
                    ..default()
                },
                ..default()
                },
                collider: Collider {
                    size: vertical_wall_size
                },
            });
        }
    }
}


fn move_paddle(
    input: Res<Input<KeyCode>>,
    time_step: Res<FixedTime>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let mut paddle_transform = query.single_mut();
    //its juts useful when we have just one entity

    let mut direction_x = 0.0;

    if input.pressed(KeyCode::A) {
        direction_x -= 1.0;
    }

    if input.pressed(KeyCode::D) {
        direction_x += 1.0;
    }

    let new_x = paddle_transform.translation.x
        + direction_x * PADDLE_SPEED * time_step.period.as_secs_f32();

    paddle_transform.translation.x = new_x;

    //y vertice
    let mut direction_y = 0.0;

    if input.pressed(KeyCode::S) {
        direction_y -= 1.0;
    }

    if input.pressed(KeyCode::W) {
        direction_y += 1.0;
    }

    let new_y = paddle_transform.translation.y
        + direction_y * PADDLE_SPEED * time_step.period.as_secs_f32();

    paddle_transform.translation.y = new_y;
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_setp: Res<FixedTime>) {
    let dt = time_setp.period.as_secs_f32();

    //its better way whe we have multiples entities
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x + dt;
        transform.translation.y += velocity.y + dt;
    }
}
