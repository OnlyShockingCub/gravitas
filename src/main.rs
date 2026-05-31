mod body;
mod physics;

use body::Body;
use macroquad::prelude::*;

#[macroquad::main("gravitas")]
async fn main() {
    let mut ball = Body {
        x: 300.0,
        y: 100.0,
        vx: 200.0,
        vy: 0.0,
        r: 20.0,
    };

    let gravity = 900.0;
    let restitution = 0.7;

    loop {
        let dt = get_frame_time();

        physics::step(
            &mut ball, 
            dt, 
            gravity);

        physics::handle_bounds(
            &mut ball,
            screen_width(),
            screen_height(),
            restitution,
        );

        clear_background(BLACK);

        draw_circle(ball.x, ball.y, ball.r, RED);

        next_frame().await;
    }
}
