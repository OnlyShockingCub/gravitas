use macroquad::prelude::*;

struct Body {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32
}

#[macroquad::main("gravitas")]

async fn main() {
    let mut ball = Body {
        x: 0.0,
        y: 10.0,
        vx: 1.0,
        vy: 0.0,
    };

    loop {
        let dt = 1.0;

        for step in 0..10 {
            ball.x += ball.vx * dt;
            ball.y += ball.vy * dt;
        }

        clear_background(BLACK);

        draw_circle(ball.x, ball.y, 20.0, RED);

        next_frame().await;
    }
}
