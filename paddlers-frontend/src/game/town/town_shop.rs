use paddlers_shared_lib::api::shop::*;
use crate::gui::{
    utils::*,
    gui_components::*,
};
use quicksilver::prelude::*;
use crate::gui::{
    input::Grabbable,
    sprites::WithSprite
};
use paddlers_shared_lib::prelude::*;

#[derive(Clone)]
pub struct DefaultShop {
    pub ui: UiBox,
}
impl Default for DefaultShop {
    fn default() -> Self {
        DefaultShop {
            ui : UiBox::new(3, 3, 4.0, 8.0)
        }
    }
}
impl DefaultShop {
    pub fn new() -> Self {
        let mut result = DefaultShop::default();
        result.add_building(BuildingType::BlueFlowers);
        result.add_building(BuildingType::RedFlowers);
        result.add_building(BuildingType::Tree);
        result.add_building(BuildingType::BundlingStation);
        result.add_building(BuildingType::SawMill);
        result.add_building(BuildingType::PresentA);
        result.add_building(BuildingType::PresentB);
        result
    }

    fn add_building(&mut self, b: BuildingType) {
        self.ui.add(
            UiElement::new(b)
                .with_image(b.sprite())
                .with_background_color(LIGHT_BLUE)
                .with_cost(b.cost())
        );
    }

    pub fn click(&self, mouse: impl Into<Vector>) -> Option<Grabbable> {
        let buy_this = self.ui.click(mouse.into());
        if let Some(ClickOutput::BuildingType(building_type)) = buy_this {
            return Some(
                Grabbable::NewBuilding(building_type)
            )
        }
        None
    }
}