//! Different utilities for drawing

use std::mem;

use sdl2::rect::Point;
use sdl2::render::{Canvas, BlendMode};
use sdl2::video::Window;

/// Draw an antialiased line.
///
/// Stolen from https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm
pub fn line_aa(canvas: &mut Canvas<Window>, mut start: (f64, f64), mut end: (f64, f64)) {
    let steep = (start.1 - end.1).abs() > (start.0 - end.0).abs();

    if steep {
        mem::swap(&mut start.0, &mut start.1);
        mem::swap(&mut end.0, &mut end.1);
    }
    if start.0 > end.0 {
        mem::swap(&mut start.0, &mut end.0);
        mem::swap(&mut start.1, &mut end.1);
    }

    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    let grad = if dx == 0. { 1. } else { dy / dx };

    // handle first endpoint
    let xend = start.0.round();
    let yend = start.1 + grad * (xend - start.0);
    let xgap = rfpart(start.0 + 0.5);
    let xpxl1 = xend;
    let ypxl1 = yend.floor();

    if steep {
        put_pixel(canvas, (ypxl1, xpxl1), rfpart(yend) * xgap);
        put_pixel(canvas, (ypxl1 + 1., xpxl1), fpart(yend) * xgap);
    } else {
        put_pixel(canvas, (xpxl1, ypxl1), rfpart(yend) * xgap);
        put_pixel(canvas, (xpxl1, ypxl1 + 1.), fpart(yend) * xgap);
    }

    let mut intery = yend + grad;

    // handle second endpoint
    let xend = end.0;
    let yend = end.1 + grad * (xend - end.0);
    let xgap = fpart(end.0 + 0.5);
    let xpxl2 = xend;
    let ypxl2 = yend.floor();

    if steep {
        put_pixel(canvas, (ypxl2, xpxl2), rfpart(yend) * xgap);
        put_pixel(canvas, (ypxl2 + 1., xpxl2), fpart(yend) * xgap);
    } else {
        put_pixel(canvas, (xpxl2, ypxl2), rfpart(yend) * xgap);
        put_pixel(canvas, (xpxl2, ypxl2 + 1.), fpart(yend) * xgap);
    }

    // main loop
    if steep {
        for x in (xpxl1 as i32 + 1)..(xpxl2 as i32 - 1) {
            put_pixel(canvas, (intery.floor(), x as f64), rfpart(intery));
            put_pixel(canvas, (intery.floor() + 1., x as f64), fpart(intery));
            intery = intery + grad;
        }
    } else {
        for x in (xpxl1 as i32 + 1)..(xpxl2 as i32 - 1) {
            put_pixel(canvas, (x as f64, intery.floor()), rfpart(intery));
            put_pixel(canvas, (x as f64, intery.floor() + 1.), fpart(intery));
            intery = intery + grad;
        }
    }
}

fn put_pixel(canvas: &mut Canvas<Window>, at: (f64, f64), intensity: f64) {
    let color_orig = canvas.draw_color();
    let mut color = color_orig.clone();

    canvas.set_blend_mode(BlendMode::Blend);

    color.a = (intensity * 256.) as u8;

    canvas.set_draw_color(color);

    canvas.draw_point(Point::new(at.0 as i32, at.1 as i32)).expect("Can't draw");

    canvas.set_draw_color(color_orig);
}

fn fpart(x: f64) -> f64 {
    x - x.floor() as f64
}
fn rfpart(x: f64) -> f64 {
    1. - fpart(x)
}
