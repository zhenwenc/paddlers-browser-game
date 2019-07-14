use quicksilver::prelude::*;
use crate::gui::{
    sprites::{Sprites, SpriteIndex},
    z::{Z_TILE_SHADOW, Z_TEXTURE}
};

#[derive(Debug)]
pub struct Town {
    map: TileMap,
    ul: f32,
}

type TileMap = [[TileType; Y]; X];
#[derive(PartialEq, Eq,Clone, Copy, Debug)]
enum TileType {
    EMPTY,
    LANE,
}
pub type TileIndex = (usize,usize);

pub const X: usize = 23;
const Y: usize = 13;
pub const TOWN_RATIO: f32 = X as f32 / Y as f32;

impl Town {
    pub fn new(ul: f32) -> Self {
        let mut map = [[TileType::EMPTY; Y]; X];
        for x in 0..X {
            for y in 0..Y {
                if y == (Y - 1) / 2 {
                    map[x][y] = TileType::LANE;
                }
            }
        }
        Town {
            map: map,
            ul: ul,
        }
    }

    pub fn update_ul(&mut self, ul: f32) {
        self.ul = ul;
    }

    pub fn render(&self, window: &mut Window, sprites: &Sprites, tick: u32, unit_length: f32) -> Result<()> {
        let d = unit_length;
        window.clear(Color::WHITE)?;

        for (x, col) in self.map.iter().enumerate() {
            for (y, tile) in col.iter().enumerate() {
                match tile {
                    TileType::EMPTY => {
                        // println!("Empty {} {}", x, y);
                        window.draw_ex(
                            &Rectangle::new((d * x as f32, d * y as f32), (d, d)),
                            Img(&sprites[SpriteIndex::Grass]),
                            Transform::IDENTITY,
                            Z_TEXTURE
                        );
                    }

                    TileType::LANE => {
                        // println!("Lane {} {}", x, y);
                        let shifted = ((tick / 10) % (d as u32)) as i32;
                        window.draw_ex(
                            &Rectangle::new((d * x as f32, d * y as f32), (d, d))
                            .translate((shifted,0)),
                            Img(&sprites[SpriteIndex::Water]),
                            Transform::IDENTITY,
                            Z_TEXTURE
                        );
                        // XXX: Hack only works for basic map
                        if x == 0 {
                            let x = -1;
                            window.draw_ex(
                                &Rectangle::new((d * x as f32, d * y as f32), (d, d))
                                .translate((shifted,0)),
                                Img(&sprites[SpriteIndex::Water]),
                                Transform::IDENTITY,
                                Z_TEXTURE
                            );
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn shadow_rectified_circle(&self, window: &mut Window, center: impl Into<Vector>, radius: f32){
        let tile = Self::find_tile(center, self.ul);
        for (x,y) in self.tiles_in_rectified_circle(tile, radius) {
            self.shadow_tile(window, (x,y));
        }
    }

    pub fn get_empty_tile(&self, pos: impl Into<Vector>, ul: f32) -> Option<(usize,usize)> {
        let (x,y) = Self::find_tile(pos, ul);
        let tile = self.map.get(x).and_then(|m| m.get(y));
        if let Some(TileType::EMPTY) = tile {
            Some((x,y))
        }
        else {
            None
        }
    }
    fn tiles_in_rectified_circle(&self, tile: TileIndex, radius: f32) -> Vec<TileIndex> {
        let r = radius.ceil() as usize;
        let xmin =  tile.0.saturating_sub(r);
        let ymin =  tile.1.saturating_sub(r);
        let xmax = if tile.0 + r + 1 > X { X } else { tile.0 + r + 1 };
        let ymax = if tile.1 + r + 1 > Y { Y } else { tile.1 + r + 1 };
        let mut tiles = vec![];
        for x in xmin .. xmax {
            for y in ymin .. ymax {
                if Self::are_tiles_in_range(tile, (x,y), radius) {
                    tiles.push((x,y));
                }
            }
        }
        tiles
    }
    pub fn lane_in_range(&self, pos: TileIndex, range: f32) -> Vec<TileIndex> {
        self.tiles_in_rectified_circle(pos, range).into_iter().filter( |(x,y)| self.map[*x][*y] == TileType::LANE ).collect()
    }

    pub fn find_tile(pos: impl Into<Vector>, ul: f32) -> (usize, usize) {
        let v = pos.into();
        let x = (v.x / ul) as usize;
        let y = (v.y / ul) as usize;
        (x,y)
    }

    #[inline]
    /// Range should be in unit lengths
    fn are_tiles_in_range(a: (usize, usize), b: (usize, usize), range: f32) -> bool {
        let dx = ( a.0.max(b.0) - a.0.min(b.0) ) as f32;
        let dy = ( a.1.max(b.1) - a.1.min(b.1) ) as f32;
        dx*dx + dy*dy <= range*range
    }

    fn shadow_tile(&self, window: &mut Window, coordinates: (usize,usize)) {
        let shadow_col = Color { r: 1.0, g: 1.0, b: 0.5, a: 0.3 };
        let (x,y) = coordinates;
        let pos = (x as f32 * self.ul, y as f32 * self.ul);
        let size = (self.ul, self.ul);
        let area = Rectangle::new(pos, size);
        window.draw_ex(
            &area,
            Col(shadow_col),
            Transform::IDENTITY, 
            Z_TILE_SHADOW,
        );
    }
}