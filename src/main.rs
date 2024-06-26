#[allow(unused, dead_code)]
use bevy::{math::*, prelude::*, sprite::collide_aabb::*};

//paddle
const PADDLE_START_Y: f32 = 0.0;
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PADDLE_SPEED: f32 = 500.0;

//ball
const BALL_COLOR: Color = Color::rgb(1.0, 7.5, 0.5);
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

//bricks
const BRICK_SIZE: Vec2 = Vec2::new(100., 30.);
const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.);
const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.;
const GAP_BETWEEN_BRICKS: f32 = 5.;
const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.;
const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                move_paddle,
                apply_velocity,
                check_ball_collisions.after(apply_velocity),
            ),
        )
        .run();
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball {
    size: Vec2,
}

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
        Collider { size: PADDLE_SIZE },
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
        Ball { size: BALL_SIZE },
        Velocity(BALL_SPEED * BALL_INITIAL_DIRECTION),
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
            commands.spawn(WallBundle {
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
                    size: vertical_wall_size,
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

    let mut new_x = paddle_transform.translation.x
        + direction_x * PADDLE_SPEED * time_step.period.as_secs_f32();

    new_x = new_x.min(RIGHT_WALL - (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);
    new_x = new_x.max(LEFT_WALL + (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);
    //Adding border limits to X axys

    paddle_transform.translation.x = new_x;

    //y vertice
    let mut direction_y = 0.0;

    if input.pressed(KeyCode::S) {
        direction_y -= 1.0;
    }

    if input.pressed(KeyCode::W) {
        direction_y += 1.0;
    }

    let mut new_y = paddle_transform.translation.y
        + direction_y * PADDLE_SPEED * time_step.period.as_secs_f32();

    new_y = new_y.min(TOP_WALL + (WALL_THICKNESS - PADDLE_SIZE.y) / 0.5);
    new_y = new_y.max(BOTTOM_WALL - (WALL_THICKNESS - PADDLE_SIZE.y) / 0.5);
    //border limits to y axys

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

fn check_ball_collisions(
    mut ball_query: Query<(&mut Velocity, &Transform, &Ball)>,
    collider_query: Query<(&Transform, &Collider)>,
) {
    for (mut ball_velocity, ball_transform, ball) in &mut ball_query {
        for (transform, other) in &collider_query {
            let collision = collide(
                ball_transform.translation,
                ball.size,
                transform.translation,
                other.size,
            );

            let mut reflect_x = false;
            let mut reflect_y = false;

            if let Some(collision) = collision {
                match collision {
                    Collision::Left => reflect_x = ball_velocity.x > 0.0,
                    Collision::Right => reflect_x = ball_velocity.x < 0.0,
                    Collision::Top => reflect_y = ball_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                    Collision::Inside => {} //do nothing
                }

                if reflect_x {
                    ball_velocity.x *= -1.;
                }
                if reflect_y {
                    ball_velocity.y *= -1.;
                }
            }
        }
    }
}
