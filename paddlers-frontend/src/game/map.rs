mod map_position;
mod map_segment;
mod map_tesselation;
mod village_meta;

use crate::gui::{
    input::{Clickable},
    ui_state::*,
    render::Renderable,
    sprites::*,
    utils::*,
    z::*,
};
use map_position::*;
use map_segment::MapSegment;
use map_tesselation::*;
use quicksilver::graphics::Mesh;
use quicksilver::prelude::*;
use specs::prelude::*;

pub use map_position::MapPosition;
pub use village_meta::VillageMetaInfo;

/// Helper struct to combine private and shared map state
pub struct GlobalMap<'a> {
    /// State that is not shareable between threads
    /// It is only accessible in the central game loop.
    private: &'a mut GlobalMapPrivateState,
    /// State than can be shared with threads safely.
    /// It is used in specs systems.
    shared: specs::shred::FetchMut<'a, GlobalMapSharedState>,
}

pub struct GlobalMapPrivateState {
    grid_mesh: Mesh,
    segments: Vec<MapSegment>,
    villages: Vec<VillageMetaInfo>,
    view_width: i32,
    loaded: (i32, i32),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GlobalMapSharedState {
    /// Offset in map coordinates (1.0 = one village width)
    x_offset: f32,
    scaling: f32,
}

impl<'a> GlobalMap<'a> {
    pub fn combined<'b>(
        private: &'b mut GlobalMapPrivateState,
        shared: specs::shred::FetchMut<'b, GlobalMapSharedState>,
    ) -> GlobalMap<'b> {
        GlobalMap::<'b> { private, shared }
    }
    pub fn new(view_size: Vector) -> (GlobalMapPrivateState, GlobalMapSharedState) {
        let scaling = Self::calculate_scaling(view_size);
        let (w, h) = Self::display_shape();
        let view_port = Rectangle::new((0, 0), Vector::new(w, h) * scaling);
        let grid_mesh = tesselate_map_background(view_port, w, h);

        let map = GlobalMapPrivateState {
            grid_mesh,
            segments: vec![],
            villages: vec![],
            view_width: w,
            loaded: (0, -1),
        };
        let shared = GlobalMapSharedState {
            x_offset: 0.0,
            scaling,
        };
        (map, shared)
    }

    pub fn render(
        &mut self,
        window: &mut Window,
        sprites: &mut Sprites,
        area: &Rectangle,
    ) -> Result<()> {
        window.draw_ex(area, Col(GREEN), Transform::IDENTITY, Z_TEXTURE);

        self.apply_scaling(area.size());
        self.draw_grid(window);
        self.draw_water(window, area);
        self.draw_villages(window, sprites)?;

        Ok(())
    }
    const LOAD_AHEAD: i32 = 10;
    const LOAD_STEP: i32 = 10;
    pub fn update(&mut self) {
        let x = -self.shared.x_offset as i32;
        if self.private.loaded.0 > x - Self::LOAD_AHEAD {
            let (low, high) = (
                self.private.loaded.0 - 1 - Self::LOAD_STEP,
                self.private.loaded.0 - 1,
            );
            crate::net::request_map_read(low, high);

            self.private.loaded.0 = self.private.loaded.0.min(low);
            self.private.loaded.1 = self.private.loaded.1.max(high);
        }
        if self.private.loaded.1 < x + self.private.view_width + Self::LOAD_AHEAD {
            let (low, high) = (
                self.private.loaded.1 + 1,
                self.private.loaded.1 + 1 + Self::LOAD_STEP,
            );
            crate::net::request_map_read(low, high);

            self.private.loaded.0 = self.private.loaded.0.min(low);
            self.private.loaded.1 = self.private.loaded.1.max(high);
        }
    }
    fn draw_grid(&mut self, window: &mut Window) {
        let mut x = self.shared.x_offset % 1.0;
        if x > 0.0 {
            x -= 1.0
        }
        let t = Transform::translate((x * self.shared.scaling, 0));
        extend_transformed(window.mesh(), &self.private.grid_mesh, t);
    }
    fn draw_water(&mut self, window: &mut Window, area: &Rectangle) {
        let visible_frame = Rectangle::new(
            (-self.shared.x_offset, 0),
            area.size() / self.shared.scaling,
        );
        let t = self.view_transform();
        for segment in self.private.segments.iter_mut() {
            if segment.is_visible(visible_frame) {
                segment.apply_scaling(self.shared.scaling);
                window.flush().unwrap();
                extend_transformed(&mut window.mesh(), &segment.water_mesh, t)
            }
        }
    }
    fn draw_villages(&mut self, window: &mut Window, sprites: &mut Sprites) -> Result<()> {
        #[cfg(feature = "dev_view")]
        self.visualize_control_points(window);

        for vil in &self.private.villages {
            let (x, y) = vil.coordinates;
            // translate human-readable to nerd indexing
            let (x, y) = (x - 1, y - 1);
            let sprite_area = Rectangle::new(
                (
                    x as f32 * self.shared.scaling,
                    y as f32 * self.shared.scaling,
                ),
                (self.shared.scaling, self.shared.scaling),
            );
            draw_image(
                sprites,
                window,
                &sprite_area,
                SpriteIndex::Simple(SingleSprite::Shack),
                Z_BUILDINGS,
                FitStrategy::Center,
                self.view_transform(),
            )?;
        }
        Ok(())
    }

    fn display_shape() -> (i32, i32) {
        let w = 15; // TODO: determine dynamically what fits the viewport
        let h = paddlers_shared_lib::game_mechanics::map::MAP_H as i32;
        (w, h)
    }
    pub fn calculate_scaling(view_size: Vector) -> f32 {
        let (w, h) = Self::display_shape();
        let rx = view_size.x / w as f32;
        let ry = view_size.y / h as f32;
        rx.min(ry)
    }
    fn apply_scaling(&mut self, size: Vector) {
        let r = Self::calculate_scaling(size);
        if self.shared.scaling != r {
            scale_mesh(&mut self.private.grid_mesh, r / self.shared.scaling);
            self.shared.scaling = r;
        }
    }
    fn view_transform(&self) -> Transform {
        Transform::translate((self.shared.x_offset * self.shared.scaling, 0))
    }

    #[cfg(feature = "dev_view")]
    fn visualize_control_points(&self, window: &mut Window) {
        let pt = self.shared.scaling / 5.0;
        for seg in &self.private.segments {
            for s in &seg.streams {
                for (x, y) in s {
                    let area = Rectangle::new(
                        (
                            (self.shared.x_offset + x) * self.shared.scaling - pt / 2.0,
                            y * self.shared.scaling - pt / 2.0,
                        ),
                        (pt, pt),
                    );
                    window.draw_ex(&area, Col(Color::WHITE), Transform::rotate(45) , 1000);
                }
            } 
        }
    }
}

impl GlobalMapPrivateState {
    pub fn add_segment(
        &mut self,
        world: &mut World,
        streams: Vec<Vec<(f32, f32)>>,
        villages: Vec<VillageMetaInfo>,
        min_x: i32,
        max_x: i32,
    ) {
        let w = max_x - min_x;
        let h = paddlers_shared_lib::game_mechanics::map::MAP_H as i32;
        let mut segment = MapSegment::new(min_x, 0, w, h, streams);
        segment.tesselate_rivers();
        self.segments.push(segment);

        for village in villages.iter() {
            world
                .create_entity()
                .with(MapPosition::new(village.coordinates))
                .with(Renderable {
                    kind: RenderVariant::ImgWithColBackground(
                        SpriteSet::Simple(SingleSprite::Shack),
                        GREEN,
                    ),
                })
                .with(Clickable)
                .with((*village).clone())
                .build();
        }

        self.villages.extend(villages.into_iter());
    }
}

impl GlobalMapSharedState {
    pub fn drag(&mut self, v: Vector) {
        self.x_offset += v.x;
    }
    pub fn left_click_on_main_area<'a>(
        &mut self,
        mouse_pos: Vector,
        mut ui_state: Write<'a, UiState>,
        entities: Entities<'a>,
        position: ReadStorage<'a, MapPosition>,
        clickable: ReadStorage<'a, Clickable>,
    ) {
        let r = self.scaling;
        let map_coordinates = Vector::new(mouse_pos.x / r, mouse_pos.y / r);

        ui_state.selected_entity = map_position_lookup(map_coordinates, entities, position, clickable);
    }
}
