use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, Rectangle, Triangle},
    style::{PrimitiveStyle, PrimitiveStyleBuilder},
};
use embedded_graphics_simulator::{SimulatorDisplay, WindowBuilder};

static CIRCLE_SIZE: i32 = 32;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(304, 128));

    // let stroke = PrimitiveStyle::with_stroke(Rgb888::MAGENTA, 1);

    // let stroke_off_fill_off = PrimitiveStyleBuilder::new()
    //     .stroke_color(Rgb888::RED)
    //     .stroke_width(1)
    //     .fill_color(Rgb888::GREEN)
    //     .build();

    // let stroke_off_fill_on = PrimitiveStyleBuilder::new()
    //     .stroke_color(Rgb888::RED)
    //     .stroke_width(1)
    //     .fill_color(Rgb888::CYAN)
    //     .build();

    // Circle::new(Point::new(CIRCLE_SIZE, CIRCLE_SIZE), CIRCLE_SIZE as u32)
    //     .into_styled(stroke)
    //     .draw(&mut display)?;

    // Circle::new(Point::new(CIRCLE_SIZE, CIRCLE_SIZE), CIRCLE_SIZE as u32)
    //     .translate(Point::new(16, 16))
    //     .into_styled(stroke_off_fill_on)
    //     .draw(&mut display)?;

    // Circle::new(Point::new(CIRCLE_SIZE, CIRCLE_SIZE), CIRCLE_SIZE as u32)
    //     .translate(Point::new(CIRCLE_SIZE, CIRCLE_SIZE))
    //     .into_styled(stroke_off_fill_off)
    //     .draw(&mut display)?;

    // Rectangle::new(Point::new(0, 0), Point::new(64, 64))
    //     .translate(Point::new(96, 0))
    //     .into_styled(stroke)
    //     .draw(&mut display)?;

    // Rectangle::new(Point::new(0, 0), Point::new(64, 64))
    //     .translate(Point::new(96 + 16, 16))
    //     .into_styled(stroke_off_fill_on)
    //     .draw(&mut display)?;

    // Rectangle::new(Point::new(0, 0), Point::new(64, 64))
    //     .translate(Point::new(96 + 32, 32))
    //     .into_styled(stroke_off_fill_off)
    //     .draw(&mut display)?;

    // Triangle::new(Point::new(32, 0), Point::new(0, 64), Point::new(64, 64))
    //     .translate(Point::new(96 * 2, 0))
    //     .into_styled(stroke)
    //     .draw(&mut display)?;

    // Triangle::new(Point::new(32, 0), Point::new(0, 64), Point::new(64, 64))
    //     .translate(Point::new(96 * 2 + 16, 16))
    //     .into_styled(stroke_off_fill_on)
    //     .draw(&mut display)?;

    // Triangle::new(Point::new(32, 0), Point::new(0, 64), Point::new(64, 64))
    //     .translate(Point::new(96 * 2 + 32, 32))
    //     .into_styled(stroke_off_fill_off)
    //     .draw(&mut display)?;

    Triangle::new(Point::new(2, 2), Point::new(8, 3), Point::new(2, 8))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_width(1)
                .stroke_color(Rgb888::RED)
                .fill_color(Rgb888::GREEN)
                .build(),
        )
        .draw(&mut display)?;

    let mut window = WindowBuilder::new(&display)
        .title("Filled primitives")
        .scale(2)
        .build();
    window.show_static(&display);

    Ok(())
}
