
pub mod constants {
    use bevy::{
        math::{Vec2, Vec3},
        render::color::Color,
    };
    //paddle
    pub const PADDLE_START_Y: f32 = 0.0;
    pub const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
    pub const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
    pub const PADDLE_SPEED: f32 = 500.0;

    //ball
    pub const BALL_COLOR: Color = Color::rgb(1.0, 7.5, 0.5);
    pub const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
    pub const BALL_SIZE: Vec2 = Vec2::new(30.0, 30.0);
    pub const BALL_SPEED: f32 = 400.0;
    pub const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

    //wall
    pub const LEFT_WALL: f32 = -450.;
    pub const RIGHT_WALL: f32 = 450.;
    pub const BOTTOM_WALL: f32 = -300.;
    pub const TOP_WALL: f32 = 300.;

    pub const WALL_THICKNESS: f32 = 10.;
    pub const WALL_BLOCK_WIDTH: f32 = RIGHT_WALL - LEFT_WALL;
    pub const WALL_BLOCK_HEIGHT: f32 = TOP_WALL - BOTTOM_WALL;
    pub const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

    //bricks
    pub const BRICK_SIZE: Vec2 = Vec2::new(100., 30.);
    pub const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.);
    pub const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.;
    pub const GAP_BETWEEN_BRICKS: f32 = 5.;
    pub const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.;
    pub const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.;
}
