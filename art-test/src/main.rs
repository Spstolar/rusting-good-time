use nannou::prelude::*;
use nannou::color;
// use std::cmp;
// use rand::seq::SliceRandom;

const WIN_W:u32 = 600;
const WIN_H:u32 = 600;
const SPIRAL_RADIUS:f32 = 350.0;
const N_ROTATIONS: u32 = 15;
const MAX_LENGTH: f32 = N_ROTATIONS as f32 * TAU;

struct Circle {
    center: Vector2,
    t: f32,
    diameter: f32,
    color: color::Rgba
}

fn spiral_location(&t: &f32) -> Vector2 {
    let r = 1.0 - (t / MAX_LENGTH);
    let r = SPIRAL_RADIUS * (r.powi(4));
    let center_x = t.cos() * r;
    let center_y = t.sin() * r;

    pt2(center_x, center_y)
}

struct CircleSpiral {
    circles: Vec<Circle>,
    distance_traveled: f32,
    next_circle: f32
}

impl CircleSpiral {
    pub fn new(circles: Vec<Circle>, distance_traveled: f32, next_circle: f32) -> Self {
        CircleSpiral{ circles, distance_traveled, next_circle }
    }

    fn add_circle(&mut self) {
        let spiral_distance = self.next_circle;
        self.distance_traveled = 0.0;
        // remove the last circle
        self.circles.pop();
        // add new circle
        let start_pt = pt2(SPIRAL_RADIUS, 0.0);
        let end_pt = spiral_location(&spiral_distance);
        let center_t = spiral_distance / 2.0;
        let center_pt = spiral_location(&center_t);

        let diameter = end_pt.distance(start_pt);

        let r = random_f32();
        let g = random_f32();
        let b = random_f32();
        let rgba = srgba(r, g, b, 1.0);

        let new_circle = Circle{ 
            center: center_pt, 
            t: center_t,
            diameter: diameter, 
            color: rgba
        };
        self.circles.insert(0, new_circle);

        let max_spiral_draw = TAU * 15.0 / 360.0;
        self.next_circle = max_spiral_draw * random_f32();
    }
}


fn gen_spiral() -> CircleSpiral {
    let mut circles: Vec<Circle> = vec!();
    let mut num_circles = 0;
    // use no more than 15 degrees for random circles
    let max_spiral_draw = TAU * 15.0 / 360.0;
    let mut spiral_t = 0.0;

    while spiral_t < MAX_LENGTH {
        // for testing, we limit the max number of circles drawn
        // in case a rule results in too many
        num_circles += 1;
        if num_circles > 1000 {
            spiral_t = MAX_LENGTH;
        }

        // we have spiral_length amount of spiral to spend
        // 1. draw a random amount of it
        // 2. calculate the center point for the circle
        // 3. calculate the diameter for the circle
        let random_length = max_spiral_draw * random_f32();

        let start_pt = spiral_location(&spiral_t);

        let center_t = spiral_t + random_length / 2.0;
        let center_pt = spiral_location(&center_t);

        let end_t = spiral_t + random_length;
        let end_pt = spiral_location(&end_t);

        spiral_t += random_length;

        let diameter = end_pt.distance(start_pt);

        let r = random_f32();
        let g = random_f32();
        let b = random_f32();
        let rgba = srgba(r, g, b, 1.0);

        circles.push(
            Circle{
                center: center_pt,
                t: center_t,
                diameter: diameter,
                color: rgba,
        });
    };

    CircleSpiral::new(circles, 0.0, max_spiral_draw)
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    circle_spiral: CircleSpiral
}

fn throb_amount(frame_num: u64, throb_length: u64, max_expand: f32) -> f32 {
    let frame = frame_num % throb_length;
    let throb_loc = frame as f32 / throb_length as f32;
    max_expand * ((0.5 - throb_loc).powi(2) * (-4.0) + 1.0)
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .title("Circle Spiral")
        .size(WIN_W, WIN_H)
        .view(view)
        .build()
        .unwrap();
    
    let circle_spiral = gen_spiral();

    Model { _window,
            circle_spiral }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let num_updates_per_rotation = 200;
    let distance_to_travel = TAU / num_updates_per_rotation as f32;
    // let shrink_rate = 1.0 - travel_distance.powf(1.95);
    let shrink_rate = 1.0 - distance_to_travel.powi(2);
    for circle in model.circle_spiral.circles.iter_mut() {
        circle.t += distance_to_travel;
        circle.center = spiral_location(&circle.t);
        circle.diameter *= shrink_rate;
    }
    model.circle_spiral.distance_traveled += distance_to_travel;
    if  model.circle_spiral.distance_traveled >= model.circle_spiral.next_circle {
        model.circle_spiral.add_circle();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    // if app.elapsed_frames() == 1{
    //     draw.background().color(BLACK);
    // }
    // draw.rect()
    //     .w_h(WIN_W as f32, WIN_H as f32)
    //     .color(srgba(0.0, 0.0, 0.0, 0.2));

    let frame_num = app.elapsed_frames();
    let throb_factor = 1.0 + throb_amount(frame_num, 20, 1.0);

    for circle in model.circle_spiral.circles.iter() {
        // draw.ellipse()
        //     .xy(circle.center * 0.8 * -1.0)
        //     .w(circle.diameter * 0.5 * throb_factor)
        //     .h(circle.diameter * 0.5 * throb_factor)
        //     .color(srgba(1.0, 1.0, 1.0, 0.2));

        draw.ellipse()
            .xy(circle.center)
            .w(circle.diameter * 0.5 * throb_factor)
            .h(circle.diameter * 0.5 * throb_factor)
            .color(circle.color);
   }

    draw.to_frame(app, &frame).unwrap();
}
