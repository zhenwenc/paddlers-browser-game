//! High-level GUI components that may be related to game logic as far as necessary

mod ui_box;
use crate::game::story::scene::SlideButtonAction;
pub use ui_box::*;
mod resources_component;
pub use resources_component::*;

use crate::game::game_event_manager::GameEvent;
use crate::gui::{sprites::*, utils::*, z::*};
use crate::prelude::*;
use paddlers_shared_lib::api::shop::Price;
use paddlers_shared_lib::prelude::AbilityType;
use quicksilver::prelude::*;

pub enum TableRow<'a> {
    Text(String),
    TextWithImage(String, SpriteIndex),
    InteractiveArea(&'a mut dyn InteractiveTableArea),
    ProgressBar(Color, Color, i32, i32, Option<String>),
}

/// An area that is part of the graphical user interface.
pub trait InteractiveTableArea {
    /// Defines how many table rows it takes to draw the area
    fn rows(&self) -> usize;
    /// Draw the area on a specified area
    fn draw(
        &mut self,
        window: &mut Window,
        sprites: &mut Sprites,
        tp: &mut TableTextProvider,
        now: Timestamp,
        area: &Rectangle,
    ) -> PadlResult<()>;
    /// Check if the mouse hits somthing on the area
    fn click(&self, mouse: Vector) -> PadlResult<Option<(ClickOutput, Option<Condition>)>>;
    /// Remove one of the clickable options
    fn remove(&mut self, output: ClickOutput);
}

#[derive(Clone, Debug, PartialEq)]
/// Elements than can be produces by a click in a interactive area
pub enum ClickOutput {
    Entity(specs::Entity),
    BuildingType(BuildingType),
    Ability(AbilityType),
    Event(GameEvent),
    SlideAction(SlideButtonAction),
}
#[derive(Clone, Debug)]
/// Represents a checkable condition. Used to check it later when the state is not available inside a system, for example.
pub enum Condition {
    HasResources(Price),
}

#[derive(Clone, Debug, Copy)]
pub enum TableVerticalAlignment {
    Top,
    Center,
}

pub struct TableTextProvider {
    text_pool: TextPool,
    white_text_pool: TextPool,
}
impl TableTextProvider {
    pub fn new() -> Self {
        TableTextProvider {
            text_pool: TextPool::default(),
            white_text_pool: TextPool::new(
                "".to_owned(),
                &[("color", "white")],
                &[],
                Rectangle::default(),
            ),
        }
    }
    pub fn new_styled(class: &'static str) -> Self {
        TableTextProvider {
            text_pool: TextPool::new("".to_owned(), &[], &[class], Rectangle::default()),
            white_text_pool: TextPool::new(
                "".to_owned(),
                &[("color", "white")],
                &[class],
                Rectangle::default(),
            ),
        }
    }
    pub fn reset(&mut self) {
        self.white_text_pool.reset();
        self.text_pool.reset();
    }
    pub fn finish_draw(&mut self) {
        self.white_text_pool.finish_draw();
        self.text_pool.finish_draw();
    }
    pub fn hide(&mut self) {
        self.white_text_pool.hide();
        self.text_pool.hide();
    }
}

pub fn draw_table(
    window: &mut Window,
    sprites: &mut Sprites,
    table: &mut [TableRow],
    max_area: &Rectangle,
    text_provider: &mut TableTextProvider,
    max_row_height: f32,
    z: i32,
    now: Timestamp,
    alignment: TableVerticalAlignment,
) -> PadlResult<()> {
    let total_rows = row_count(table);
    let row_height = max_row_height.min(max_area.height() / total_rows as f32);
    let font_h = row_height * 0.9;
    let img_s = row_height * 0.95;
    let margin = 10.0;
    let mut line = Rectangle::new(max_area.pos, (max_area.width(), row_height));
    match alignment {
        TableVerticalAlignment::Top => { /* NOP */ }
        TableVerticalAlignment::Center => {
            let table_h = total_rows as f32 * row_height;
            let shift = (max_area.height() - table_h) / 2.0;
            line.pos.y += shift;
        }
    }
    for row in table {
        let floats = &mut text_provider.text_pool;
        let white_floats = &mut text_provider.white_text_pool;
        match row {
            TableRow::Text(text) => {
                let mut text_area = line.clone();
                text_area.size.y = font_h;
                floats
                    .allocate()
                    .write(window, &text_area, z, FitStrategy::Center, text)?;
                line.pos.y += row_height;
            }
            TableRow::TextWithImage(text, img) => {
                let symbol = Rectangle::new(line.pos, (img_s, img_s));
                let mut text_area = line.clone();
                let shift_x = img_s + margin;
                text_area.size.x -= shift_x;
                text_area.pos.x += shift_x;
                text_area.size.y = font_h;
                text_area.pos.y += row_height - font_h; // something is fishy here, should be /2.0 but right now looks better without
                floats
                    .allocate()
                    .write(window, &text_area, z, FitStrategy::Center, text)?;
                draw_static_image(sprites, window, &symbol, *img, z, FitStrategy::Center)?;
                line.pos.y += row_height;
            }
            TableRow::InteractiveArea(ia) => {
                let mut area = line.clone();
                area.size.y = area.size.y * ia.rows() as f32;
                ia.draw(window, sprites, text_provider, now, &area)?;
                line.pos.y += area.size.y;
            }
            TableRow::ProgressBar(bkgcol, col, i, n, label) => {
                let mut area = line.clone();
                let margin = area.size.y * 0.15;
                area.size.y -= margin;
                if let Some(label) = label {
                    let mut label_area = area.clone();
                    let w = label_area.size.y;
                    label_area.size.x = w;
                    area.pos.x += w;
                    area.size.x -= w;
                    window.draw_ex(&label_area, Col(*bkgcol), Transform::IDENTITY, z);
                    let mut label_text_area = label_area.shrink_to_center(0.9);
                    label_text_area.pos.y += label_text_area.size.y * 0.1;
                    white_floats.allocate().write(
                        window,
                        &label_text_area,
                        z,
                        FitStrategy::Center,
                        label,
                    )?;
                }
                let text = format!("{}/{}", i, n);
                let mut text_area = area.shrink_to_center(0.9);
                text_area.pos.y += text_area.size.y * 0.1;
                white_floats.allocate().write(
                    window,
                    &text_area,
                    z + 1,
                    FitStrategy::Center,
                    &text,
                )?;

                window.draw_ex(&area, Col(*col), Transform::IDENTITY, Z_MENU_BOX + 1);
                let mut bar = area.padded(3.0);
                window.draw_ex(&bar, Col(*bkgcol), Transform::IDENTITY, Z_MENU_BOX + 2);
                bar.size.x *= *i as f32 / *n as f32;
                window.draw_ex(&bar, Col(*col), Transform::IDENTITY, Z_MENU_BOX + 3);
                line.pos.y += line.size.y;
            }
        }
    }
    Ok(())
}

fn row_count(table: &[TableRow]) -> usize {
    table.iter().fold(0, |acc, row| {
        acc + match row {
            TableRow::Text(_) => 1,
            TableRow::TextWithImage(_, _) => 1,
            TableRow::InteractiveArea(ia) => ia.rows(),
            TableRow::ProgressBar(_, _, _, _, _) => 1,
        }
    })
}

impl From<specs::Entity> for ClickOutput {
    fn from(e: specs::Entity) -> Self {
        ClickOutput::Entity(e)
    }
}
impl From<BuildingType> for ClickOutput {
    fn from(bt: BuildingType) -> Self {
        ClickOutput::BuildingType(bt)
    }
}
impl From<AbilityType> for ClickOutput {
    fn from(a: AbilityType) -> Self {
        ClickOutput::Ability(a)
    }
}
impl From<GameEvent> for ClickOutput {
    fn from(evt: GameEvent) -> Self {
        ClickOutput::Event(evt)
    }
}
