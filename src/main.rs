use nannou::prelude::*;
use nannou::rand;
use nannou::color;

const DIMENSION: usize = 3;

#[derive(Clone)]
struct Object {
    pub position: [f32; DIMENSION],
    pub velocity: [f32; DIMENSION],
    pub masse: f32,
}

struct Model {
    pub started: u32,
    pub objects: Vec<Object>,
}

const ZOOM: f32 = 0.1;
const MASS_SIZE: f32 = 0.3;
const MASS_SIZE_ZOOM: f32 = 1.;
const G: f32 = 1.;

const INIT_POSITION_RANGE: f32 = 1000.;
const INIT_VELOCITY_RANGE: f32 = 5.;

const ELASTIC_COLLISION_FACTOR: f32 = 0.;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    let mut objects = vec![
    ];

    for _ in 0..500 {
        let masse = rand::random_range(8_000., 10_000.);

        objects.push(
            Object {
                position: [1000. + rand::random_range(-INIT_POSITION_RANGE, INIT_POSITION_RANGE), 1300. + rand::random_range(-INIT_POSITION_RANGE, INIT_POSITION_RANGE), 1000. + rand::random_range(-INIT_POSITION_RANGE, INIT_POSITION_RANGE)],
                velocity: [rand::random_range(-INIT_VELOCITY_RANGE, INIT_VELOCITY_RANGE), rand::random_range(-INIT_VELOCITY_RANGE, INIT_VELOCITY_RANGE), rand::random_range(-INIT_VELOCITY_RANGE, INIT_VELOCITY_RANGE)],
                masse: masse,
            }
        );
    }

    for _ in 0..500 {
        let masse = rand::random_range(8_000., 10_000.);

        objects.push(
            Object {
                position: [-1000. + rand::random_range(-INIT_POSITION_RANGE, INIT_POSITION_RANGE), 1300. + rand::random_range(-INIT_POSITION_RANGE, INIT_POSITION_RANGE), 1000. + rand::random_range(-INIT_POSITION_RANGE, INIT_POSITION_RANGE)],
                velocity: [rand::random_range(-INIT_VELOCITY_RANGE, INIT_VELOCITY_RANGE), rand::random_range(-INIT_VELOCITY_RANGE, INIT_VELOCITY_RANGE), rand::random_range(-INIT_VELOCITY_RANGE, INIT_VELOCITY_RANGE)],
                masse: masse,
            }
        );
    }

    for _ in 0..1000 {
        let masse = rand::random_range(8_000., 10_000.);

        objects.push(
            Object {
                position: [rand::random_range(-INIT_POSITION_RANGE, INIT_POSITION_RANGE), -1300. + rand::random_range(-INIT_POSITION_RANGE, INIT_POSITION_RANGE), rand::random_range(-INIT_POSITION_RANGE, INIT_POSITION_RANGE)],
                velocity: [rand::random_range(-INIT_VELOCITY_RANGE, INIT_VELOCITY_RANGE), rand::random_range(-INIT_VELOCITY_RANGE, INIT_VELOCITY_RANGE), rand::random_range(-INIT_VELOCITY_RANGE, INIT_VELOCITY_RANGE)],
                masse: masse,
            }
        );
    }
    return Model { started: 0, objects: objects };
}

fn sq_distance_between(object1: &[f32; DIMENSION], object2: &[f32; DIMENSION]) -> f32 {
    let mut r = 0.;

    for d in 0..DIMENSION {
        r += (object2[d] - object1[d]).powi(2);
    }

    return r;
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for i in 0..model.objects.len() {
        if model.objects[i].masse == 0. {
            continue;
        }
        for d in 0..DIMENSION {
            model.objects[i].position[d] += model.objects[i].velocity[d] * 0.01;
        }
        for j in 0..model.objects.len() {
            if j == i || model.objects[j].masse == 0. {
                continue;
            }

            let r2 = sq_distance_between(&model.objects[i].position, &model.objects[j].position);
            let r = r2.sqrt();

            let force = G * model.objects[i].masse * model.objects[j].masse / r2;

            let mut dir = [0.; DIMENSION];

            for d in 0..DIMENSION {
                dir[d] = (model.objects[i].position[d] - model.objects[j].position[d]) / r;
            }

            let a = force / model.objects[i].masse;

            if r < model.objects[i].masse.cbrt() * MASS_SIZE / 2.  + model.objects[j].masse.cbrt() * MASS_SIZE  / 2. {
                continue;
            }

            for d in 0..DIMENSION {
                model.objects[i].velocity[d] -= dir[d] * a;
            }
        }
    }
    model.started += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if model.started < 10 {
        draw.background().color(BLACK);
    }

    for i in 0..model.objects.len() {
        let object = &model.objects[i];
        if object.position[2] < -4_500. || object.position[2] > 5_000. || object.position[0].is_nan() || object.position[1].is_nan() || object.position[2].is_nan() {
            continue;
        }
        draw.ellipse()
            .color(color::lin_srgba(1., 1., 1., 0.01))
            .w(object.masse.cbrt() * ZOOM * MASS_SIZE * MASS_SIZE_ZOOM * 5_000. / (object.position[2] + 5_000.))
            .h(object.masse.cbrt() * ZOOM * MASS_SIZE * MASS_SIZE_ZOOM * 5_000. / (object.position[2] + 5_000.))
            .x_y(object.position[0] * ZOOM - object.position[2] * 0.2 * ZOOM, object.position[1] * ZOOM + object.position[2] * 0.2 * ZOOM);
    }

    draw.to_frame(app, &frame).unwrap();
}
