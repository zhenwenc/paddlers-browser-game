use super::*;
use quicksilver::prelude::*;

pub fn draw_progress_bar(
    window: &mut Window,
    float: &mut FloatingText,
    area: Rectangle,
    progress: f32,
    text: &str,
) -> PadlResult<()> {
    let text_h = (area.height() * 0.5).max(50.0);
    let (text_area, bar_area) = area.cut_horizontal(text_h);

    let z = 1;

    float.write(
        window,
        &text_area.padded(10.0),
        z,
        FitStrategy::Center,
        text,
    )?;

    window.draw(&bar_area, Col(Color::WHITE));
    let mut bar = bar_area.padded(3.0);
    window.draw(&bar, Col(DARK_GREEN));
    bar.size.x *= progress;
    window.draw(&bar, Col(LIGHT_GREEN));
    Ok(())
}
