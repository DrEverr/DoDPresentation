use rand::Rng;
pub struct World {
    width: f64,
    height: f64,
    pub positions: Vec<(f64, f64)>,
    velocities: Vec<(f64, f64)>,
}

pub fn init(amount: usize, width: f64, height: f64) -> World {
    let mut rng = rand::thread_rng();

    let mut positions = Vec::with_capacity(amount);
    let mut velocities = Vec::with_capacity(amount);

    for _ in 0..amount {
        positions.push((rng.gen_range(0.0, width), rng.gen_range(0.0, height)));
        velocities.push((rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0)));
    }

    World {
        width,
        height,
        positions,
        velocities,
    }
}

pub fn animate(world: &mut World) {
    for (pos, vel) in world.positions.iter_mut().zip(world.velocities.iter_mut()) {
        *pos = (pos.0 + vel.0, pos.1 + vel.1);

        if pos.0 < 0.0 {
            pos.0 = 0.0;
            vel.0 *= -1.0;
        }
        if pos.0 > world.width {
            pos.0 = world.width;
            vel.0 *= -1.0;
        }
        if pos.1 > world.height {
            pos.1 = world.height;
            vel.1 *= -1.0;
        }
        if pos.1 < 0.0 {
            pos.1 = 0.0;
            vel.1 *= -1.0;
        }
    }
}