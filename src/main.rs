//! Draw some animated squigly lines using Nannou
//!
//! Nannou guide is here: https://guide.nannou.cc/
//! Nannou API reference is here: https://docs.rs/nannou/0.19.0/nannou/
//! Nannou examples can be found here: https://github.com/nannou-org/nannou/tree/master/examples
//!
//! This code is based on https://github.com/nannou-org/nannou/blob/master/examples/draw/draw_polyline.rs

use std::cmp::max;
// Include the "prelude" from Nannou (standard set of functions and types)
use nannou::noise::*;
use nannou::prelude::*;
use nannou::event::Event::*;
use nannou::event::TouchpadPressure;
use nannou::Event::WindowEvent;
use nannou::winit::event::DeviceId;


struct Model {
    heihgt: f32,
    prev_pressure: f32,
    prev_stage: f32,
}


/// `main` is the default entry point for Rust programs
fn main() {
    // Create a simple app and tells Nannou to call the `view_update` function
    //     https://docs.rs/nannou/latest/nannou/fn.sketch.html
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .run();

}

fn event(_app: &App, _model: &mut Model, event: Event) {
    match event {
        WindowEvent { simple: Some(window_event), .. } => {
            match window_event {
                TouchPressure(pressure) => {
                    println!("Touchpad pressure: {:?}", pressure);

                    if _model.prev_stage != pressure.stage as f32 {
                        _model.prev_pressure = pressure.pressure;
                    }

                    let norm_pressure =  f32::max(pressure.pressure, 0.1);
                    let actual_pressure = norm_pressure * (pressure.stage as f32 + 1.0);
                    _model.heihgt = f32::max(actual_pressure/2.0, 0.1);

                    _model.prev_pressure = pressure.pressure;
                    _model.prev_stage = pressure.stage as f32;
                }
                _ => (),
            }
        }
        _ => (),
    }
}

fn model(_app: &App) -> Model {
    Model {
        heihgt: 0.0,
        prev_pressure: 0.0,
        prev_stage: 0.0,
    }
}

/// `view_update` is called periodically by Nannou to update the drawing
/// `app` is passed from Nannou with information about the current application state
///     https://docs.rs/nannou/latest/nannou/app/struct.App.html
/// `frame` is where well finally send our drawing to, which Nannou will display on screen
///     https://docs.rs/nannou/latest/nannou/frame/struct.Frame.html
fn view(_app: &App, _model: &Model, _frame: Frame) {
    let noise = nannou::noise::HybridMulti::new();

    // Get the rectangle (size) of the window
    let win = _app.window_rect();

    // Get the number of seconds (with fractions) since the app started
    let time = _app.time;

    // Get an object from nannou that let's us easily draw shapes
    let draw = _app.draw();

    // Set the background to black
    draw.background().color(BLACK);

    // How many sine wave cycles to show
    const HZ: f32 = 1.0;

    const LINES: usize = 20;
    for line_number in 0..LINES {
        // `map_range`: Maps a value from an input range to an output range

        // Height of each line, should be bigger than the distance between each line, to get some overlap
        let line_height = _model.heihgt;

        // Start drawing lines at top of screen (y=1.0) and  work our way down
        let line_position = map_range(line_number, 0, LINES, 1.0, -1.0);

        // Create an array with 500 2D points (`Vec2`s) which will define a single line in our art
        let mut points = [Vec2::ZERO; 500];
        let first_index = 0;
        let last_index = points.len() - 1;

        // Iterate through all the points in the above array, with the index number
        for (index, point_ref) in points.iter_mut().enumerate() {
            // Calculate the `x` position from the index
            // first point in array = -1.0 = left of line
            // last  point in array =  1.0 = right of line
            let x = map_range(index, first_index, last_index, -1.0, 1.0);

            // Calculate the `y` position
            let y_noise = noise.get([(x + time) as f64, line_position as f64]) as f32;
            let y = (x * HZ * PI).sin() * y_noise;
            let y = y.clamp(0.0, 1.0) * line_height + line_position;

            // Convert the above normalized (-1.0 -> 0.0) coordinates to window coordinates
            //     https://guide.nannou.cc/tutorials/basics/window-coordinates
            let window_x = map_range(x, -1.0, 1.0, win.left(), win.right());
            let window_y = map_range(y, -1.0, 1.0, win.bottom(), win.top());

            *point_ref = pt2(window_x, window_y);
        }

        // Fill with black to obscure lines behind this one
        draw.polygon().color(BLACK).points(points);

        // Draw white outline
        draw.path()
            .stroke()
            .color(WHITE)
            .stroke_weight(2.0)
            .join_round()
            .caps_round()
            .points(points);
    }

    // Finish drawing to the frame that Nannou provided
    draw.to_frame(_app, &_frame).unwrap();
}
