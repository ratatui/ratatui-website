use ratatui::{
    prelude::*,
    widgets::canvas::{self, Canvas, Circle, Context, Rectangle},
};

pub fn render(f: &mut Frame) {
    let canvas = Canvas::default()
        .x_bounds([0., 40.])
        .y_bounds([0., 20.])
        .paint(draw);

    f.render_widget(canvas, f.size());
}

fn draw(ctx: &mut Context) {
    // Sky
    ctx.draw(&canvas::Line {
        x1: 0.,
        y1: 20.,
        x2: 32.2,
        y2: 20.,
        color: Color::LightBlue,
    });

    // Sun
    ctx.draw(&Circle {
        x: 39.,
        y: 19.,
        radius: 6.2,
        color: Color::Yellow,
    });

    // Grass
    ctx.draw(&canvas::Line {
        x1: 0.,
        y1: 1.,
        x2: 40.,
        y2: 1.,
        color: Color::Green,
    });
    ctx.layer();

    // Trunk
    ctx.draw(&Rectangle {
        x: 9.8,
        y: 1.,
        width: 2.2,
        height: 7.,
        color: Color::Rgb(106, 65, 23),
    });
    ctx.layer();

    // Leaves
    ctx.draw(&Circle {
        x: 11.,
        y: 11.2,
        radius: 3.5,
        color: Color::Green,
    })
}
