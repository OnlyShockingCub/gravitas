use crate::body::Body;

pub fn step(body: &mut Body,
    dt: f32,
    gravity: f32,
    ) {
    body.vy += gravity * dt;

    body.x += body.vx * dt;
    body.y += body.vy * dt;
}

pub fn handle_bounds(
    body: &mut Body,
    width: f32,
    height: f32,
    restitution: f32,
) {
    if body.x < body.r {
        body.x = body.r;
        body.vx = body.vx.abs() * restitution;
    }

    if body.x > width - body.r {
        body.x = width - body.r;
        body.vx = -body.vx.abs() * restitution;
    }

    if body.y < body.r {
        body.y = body.r;
        body.vy = body.vy.abs() * restitution;
    }

    if body.y > height - body.r {
        body.y = height - body.r;
        body.vy = -body.vy * restitution;

        body.vx *= 0.98;

        if body.vy.abs() < 1.0 {
            body.vy = 0.0;
        }
    }
}

