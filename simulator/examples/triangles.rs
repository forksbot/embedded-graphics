use embedded_graphics::{
    pixelcolor::Rgb888, prelude::*, primitives::Triangle, style::PrimitiveStyleBuilder,
};
use embedded_graphics_simulator::{SimulatorDisplay, SimulatorEvent, WindowBuilder};
use sdl2::keyboard::Keycode;

const PAD: i32 = 10;
const PAD_Y: i32 = 10;

fn draw(
    display: &mut SimulatorDisplay<Rgb888>,
    fill: Option<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::RED)
        .stroke_width(1);

    let style = if let Some(fill) = fill {
        style.fill_color(fill)
    } else {
        style
    };

    let style = style.build();

    // // no straight lines
    // Triangle::new(Point::new(0, 0), Point::new(64, 10), Point::new(15, 64))
    //     .translate(Point::new(PAD, PAD_Y))
    //     .into_styled(style)
    //     .draw(display)?;

    // // flat top
    // Triangle::new(Point::new(5, 0), Point::new(30, 64), Point::new(64, 0))
    //     .translate(Point::new(64 + PAD, PAD_Y))
    //     .into_styled(style)
    //     .draw(display)?;

    // // flat left
    // Triangle::new(Point::new(0, 0), Point::new(0, 64), Point::new(64, 30))
    //     .translate(Point::new((64 + PAD) * 2, PAD_Y))
    //     .into_styled(style)
    //     .draw(display)?;

    // // flat bottom
    // Triangle::new(Point::new(22, 0), Point::new(0, 64), Point::new(64, 64))
    //     .translate(Point::new((64 + PAD) * 3, PAD_Y))
    //     .into_styled(style)
    //     .draw(display)?;

    // // flat right
    // Triangle::new(Point::new(0, 22), Point::new(64, 0), Point::new(64, 64))
    //     .translate(Point::new((64 + PAD) * 4, PAD_Y))
    //     .into_styled(style)
    //     .draw(display)?;

    // draw filled above stroke, should not be visible
    Triangle::new(Point::new(0, 22), Point::new(64, 0), Point::new(64, 64))
        .translate(Point::new((64 + PAD) * 5, PAD_Y))
        .into_styled(style)
        .draw(display)?;

    // Triangle::new(Point::new(0, 22), Point::new(64, 0), Point::new(64, 64))
    //     .translate(Point::new((64 + PAD) * 5, PAD_Y))
    //     .into_styled(style)
    //     .draw(display)?;

    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(512, 128));

    let mut window = WindowBuilder::new(&display)
        .title("Triangles")
        .scale(6)
        .build();

    let mut fill = Some(Rgb888::GREEN);

    draw(&mut display, fill)?;

    'running: loop {
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown { keycode, .. } => {
                    match keycode {
                        Keycode::Space => {
                            if fill.is_some() {
                                fill = None
                            } else {
                                fill = Some(Rgb888::GREEN)
                            }

                            println!("ASDASDASD {:?}", fill);

                            display.clear(Rgb888::BLACK)?;
                            draw(&mut display, fill)?;
                        }
                        _ => (),
                    };
                }
                _ => {}
            }
        }
    }

    Ok(())
}
