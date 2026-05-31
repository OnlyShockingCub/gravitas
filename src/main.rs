use std::fs::read_to_string;

use macroquad::prelude::*;

struct Body {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    r: f32,
}

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
        let w = screen_width();
        let h = screen_height();

        ball.vy += gravity * dt;

        ball.x += ball.vx * dt;
        ball.y += ball.vy * dt;

        if ball.x < ball.r {
            ball.x = ball.r;
            ball.vx = ball.vx.abs() * restitution;
        }

        if ball.x > w - ball.r {
            ball.x = w - ball.r;
            ball.vx = -ball.vx.abs() * restitution;
        }

        if ball.y < ball.r {
            ball.y = ball.r;
            ball.vy = ball.vy.abs() * restitution;
        }

        if ball.y > h - ball.r {
            ball.y = h - ball.r;
            ball.vy = -ball.vy * restitution;

            ball.vx *= 0.98;

            if ball.vy.abs() < 1.0 {
                ball.vy = 0.0;
            }
        }

        clear_background(BLACK);
        draw_circle(ball.x, ball.y, ball.r, RED);

        next_frame().await;
    }
}
