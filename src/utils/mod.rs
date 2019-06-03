//! Different utilities for drawing

use std::mem;

use sdl2::rect::Point;
use sdl2::render::{BlendMode, Canvas};
use sdl2::video::Window;

/// Draw an antialiased line.
pub fn line_aa(canvas: &mut Canvas<Window>, start: (f64, f64), end: (f64, f64)) {
    line_aa_width(canvas, start, end, 1.);
}

/// Draw an antialiased line with a specified line width
pub fn line_aa_width(
    canvas: &mut Canvas<Window>,
    mut start: (f64, f64),
    mut end: (f64, f64),
    line_size: f64,
) {
    let steep = (start.1 - end.1).abs() > (start.0 - end.0).abs();

    if steep {
        mem::swap(&mut start.0, &mut start.1);
        mem::swap(&mut end.0, &mut end.1);
    }
    if start.0 > end.0 {
        mem::swap(&mut start, &mut end);
    }

    let dx = end.0 - start.0;
    let dy = end.1 - start.1;

    let grad = dy / dx;

    let line_height = line_size * (dx * dx + dy * dy).sqrt() / dx;

    let half_height = line_height / 2.;
    let half_size = line_size / 2.;

    for x in (start.0 - half_size).ceil() as isize..=(end.0 + half_size) as isize {
        let x = x as f64;
        let x_rel = x - start.0;
        let y_rel = x_rel * grad;
        let y = y_rel + start.1;

        let mut up = y + half_height;
        let mut down = y - half_height;

        if x_rel < half_size {
            let y_rel_sq = half_size * half_size - x_rel * x_rel;
            let y_rel = y_rel_sq.sqrt();

            if end.1 >= start.1 {
                if x_rel < 0. {
                    down = start.1 - y_rel;
                }
                if y_rel / x_rel > 1. / grad {
                    down = start.1 - y_rel;
                }
                if y_rel / x_rel > -1. / grad && x_rel < 0. {
                    up = start.1 + y_rel;
                }
            } else {
                if x_rel < 0. {
                    up = start.1 + y_rel;
                }
                if y_rel / x_rel > -1. / grad {
                    up = start.1 + y_rel;
                }
                if y_rel / x_rel > 1. / grad && x_rel < 0. {
                    down = start.1 - y_rel;
                }
            }
        }

        let x_rel = x - end.0;
        if x_rel > -half_size {
            let y_rel_sq = half_size * half_size - x_rel * x_rel;
            let y_rel = y_rel_sq.sqrt();

            if end.1 < start.1 {
                if x_rel >= 0. {
                    down = end.1 - y_rel;
                }
                if y_rel / x_rel < 1. / grad {
                    down = end.1 - y_rel;
                }
                if y_rel / x_rel < -1. / grad && x_rel >= 0. {
                    up = end.1 + y_rel;
                }
            } else {
                if x_rel >= 0. {
                    up = end.1 + y_rel;
                }
                if y_rel / x_rel < -1. / grad {
                    up = end.1 + y_rel;
                }
                if y_rel / x_rel < 1. / grad && x_rel >= 0. {
                    down = end.1 - y_rel;
                }
            }
        }

        // Fill filled points
        for y_ in down.ceil() as isize..up.floor() as isize {
            put_pixel(canvas, (x, y_ as f64), 1., steep);
        }

        put_pixel(canvas, (x, up), fpart(up), steep);
        put_pixel(canvas, (x, down), rfpart(down), steep);
    }
}

fn put_pixel(canvas: &mut Canvas<Window>, at: (f64, f64), intensity: f64, steep: bool) {
    let color_orig = canvas.draw_color();
    let mut color = color_orig.clone();

    canvas.set_blend_mode(BlendMode::Blend);

    color.a = (intensity * color.a as f64) as u8;

    canvas.set_draw_color(color);

    if steep {
        canvas
            .draw_point(Point::new(at.1 as i32, at.0 as i32))
            .expect("Can't draw");
    } else {
        canvas
            .draw_point(Point::new(at.0 as i32, at.1 as i32))
            .expect("Can't draw");
    }

    canvas.set_draw_color(color_orig);
}

fn fpart(x: f64) -> f64 {
    x - x.floor() as f64
}
fn rfpart(x: f64) -> f64 {
    1. - fpart(x)
}
