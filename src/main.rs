//! Draw some animated squigly lines using Nannou
//!
//! Nannou guide is here: https://guide.nannou.cc/
//! Nannou API reference is here: https://docs.rs/nannou/0.19.0/nannou/
//! Nannou examples can be found here: https://github.com/nannou-org/nannou/tree/master/examples
//!
//! This code is based on https://github.com/nannou-org/nannou/blob/master/examples/draw/draw_polyline.rs

// Include the "prelude" from Nannou (standard set of functions and types)
use nannou::prelude::*;

/// `main` is the default entry point for Rust programs
fn main() {
    // Create a simple app and tells Nannou to call the `view_update` function
    //     https://docs.rs/nannou/latest/nannou/fn.sketch.html
    nannou::sketch(view_update).run()
}

/// `view_update` is called periodically by Nannou to update the drawing
/// `app` is passed from Nannou with information about the current application state
///     https://docs.rs/nannou/latest/nannou/app/struct.App.html
/// `frame` is where well finally send our drawing to, which Nannou will display on screen
///     https://docs.rs/nannou/latest/nannou/frame/struct.Frame.html
fn view_update(app: &nannou::App, frame: nannou::Frame) {
    // Get the rectangle (size) of the window
    let win = app.window_rect();

    // Get the number of seconds (with fractions) since the app started
    let time = app.time;

    // Get an object from nannou that let's us easily draw shapes
    let draw = app.draw();

    // Set the background to black
    draw.background().color(BLACK);

    // How many sine wave cycles to show
    const HZ: f32 = 1.0;

    // Create an array with 500 2D points (`Vec2`s) which will define our line
    let mut points = [Vec2::ZERO; 500];
    let first_index = 0;
    let last_index = points.len() - 1;

    // Iterate through all the points in the above array, with the index number
    for (index, point_ref) in points.iter_mut().enumerate() {
        // `map_range`: Maps a value from an input range to an output range

        // Calculate the `x` position from the index
        // first point in array = -1.0 = left of line
        // last  point in array =  1.0 = right of line
        let x = map_range(index, first_index, last_index, -1.0, 1.0);

        // Calculate the `y` position
        let y = (time + x * HZ * PI).sin();

        // Convert the above normalized (-1.0 -> 0.0) coordinates to window coordinates
        //     https://guide.nannou.cc/tutorials/basics/window-coordinates
        let window_x = map_range(x, -1.0, 1.0, win.left(), win.right());
        let window_y = map_range(y, -1.0, 1.0, win.bottom(), win.top());

        *point_ref = pt2(window_x, window_y);
    }

    // Draw a stroked path of our points (like a paint brush)
    draw.path()
        .stroke()
        .color(RED)
        .stroke_weight(8.0)
        .join_round()
        .caps_round()
        .points(points);

    // Finish drawing to the frame that Nannou provided
    draw.to_frame(app, &frame).unwrap();
}
