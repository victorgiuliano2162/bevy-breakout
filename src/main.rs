#[allow(unused, dead_code)]
mod constants;

use bevy::{
    math::*,
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        RenderPlugin,
    },
    sprite::collide_aabb::*, transform::commands,
};
use constants::constants::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            wgpu_settings: WgpuSettings {
                backends: Some(Backends::VULKAN),
                ..default()
            },
        }))
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(Scoreboard { score: 0})
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

#[derive(Component)]
struct Brick;

#[derive(Resource, Clone, Copy)]
struct Scoreboard {
    score: usize
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

        //bricks
        {
            let offset_x = LEFT_WALL + GAP_BETWEEN_BRICKS_AND_SIDES + BRICK_SIZE.x * 0.5;
            let offset_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_BRICKS + BRICK_SIZE.y * 0.5;

            let bricks_total_width = (RIGHT_WALL - LEFT_WALL) - 2. * GAP_BETWEEN_BRICKS_AND_SIDES;
            let bricks_total_height = (TOP_WALL - BOTTOM_WALL)
                - GAP_BETWEEN_BRICKS_AND_CEILING
                - GAP_BETWEEN_PADDLE_AND_BRICKS;

            let rows = (bricks_total_height / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as i32;
            let columns = (bricks_total_width / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as i32;
            //convert values to i32 its just necessary to for loops ahead

            for row in 0..rows {
                for column in 0..columns {
                    let brick_pos = vec2(
                        offset_x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
                        offset_y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
                    );

                    commands.spawn((
                        SpriteBundle {
                            transform: Transform {
                                translation: brick_pos.extend(0.0),
                                ..default()
                            },
                            ..default()
                            Brick,
                            Collider { size: BRICK_SIZE },
                        },
                    ));
                }
            }
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
    mut commands: Commands,
    mut score: ResMut<Scoreboard>,
    mut ball_query: Query<(&mut Velocity, &Transform, &Ball)>,
    collider_query: Query<(Entity, &Transform, &Collider, Option<&Brick>)>,
) {
    for (mut ball_velocity, ball_transform, ball) in &mut ball_query {
        for (other_entity, transform, other, opt_brick) in &collider_query {
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

                if  opt_brick.is_some() {
                    score.score += 1;
                    println!("Score: {}", score.score);
                    commands.entity(other_entity).despawn()
                }
            }
        }
    }
}
