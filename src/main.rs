mod body;
mod physics;

use body::Body;
use macroquad::prelude::*;

#[macroquad::main("gravitas")]
async fn main() {
    let mut balls = vec![
        Body {
            x: 300.0,
            y: 100.0,
            vx: 200.0,
            vy: 0.0,
            r: 20.0,
        },
        Body {
            x: 400.0,
            y: 100.0,
            vx: 200.0,
            vy: 0.0,
            r: 20.0,
        },
    ];

    let gravity = 900.0;
    let restitution = 0.7;

    loop {
        let dt = get_frame_time();

        for ball in &mut balls {
            physics::step(
                ball,
                dt,
                gravity,
            );

            physics::handle_bounds(
                ball,
                screen_width(),
                screen_height(),
                restitution,
            );
        }

        clear_background(BLACK);

        for ball in &balls {
            draw_circle(ball.x, ball.y, ball.r, RED);
        }

        next_frame().await;
    }
}
