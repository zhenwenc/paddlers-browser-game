use super::*;
use crate::game::buildings::Building;
use crate::game::fight::Aura;
use crate::game::visits::attacks::Attack;
use crate::net::graphql::attacks_query::{AttacksQueryVillageAttacksUnits, HoboAttributeType};
use paddlers_shared_lib::game_mechanics::town::*;
use paddlers_shared_lib::graphql_types::*;
use specs::prelude::*;

pub(crate) struct AttackingHobo<'a> {
    pub unit: AttacksQueryVillageAttacksUnits,
    pub attack: &'a Attack,
}

impl<'a> IAttackingHobo for AttackingHobo<'a> {
    fn max_hp(&self) -> u32 {
        self.unit.hobo.hp as u32
    }
    fn speed(&self) -> f32 {
        self.unit.hobo.speed as f32
    }
    fn hurried(&self) -> bool {
        self.unit.hobo.hurried
    }
    fn arrival(&self) -> i64 {
        self.attack.arrival.millis()
    }
    fn released(&self) -> Option<i64> {
        self.unit.info.released.as_ref().map(|t| {
            GqlTimestamp::from_string(&t)
                .unwrap()
                .to_chrono()
                .timestamp_millis() as i64
        })
    }
    fn effects_strength(&self) -> i32 {
        self.unit
            .hobo
            .effects
            .iter()
            .filter(|e| e.attribute == HoboAttributeType::HEALTH)
            .filter(|e| e.strength.is_some())
            .fold(0, |acc, e| acc + e.strength.unwrap() as i64) as i32
    }
}

impl<'a, 'b> ITownLayoutMarker for Game<'a, 'b> {
    const LAYOUT: TownLayout = TownLayout::Basic;
}
impl<'a, 'b> IDefendingTown for Game<'a, 'b> {
    type AuraId = u32;
    fn auras_in_range(&self, index: &Self::Index, time: i64) -> Vec<(Self::AuraId, i32)> {
        let mut out = vec![];

        let auras = self.world.read_component::<Aura>();
        let buildings = self.world.read_component::<Building>();
        let entities = self.world.entities();
        for (aura, e, b) in (&auras, &entities, &buildings).join() {
            if time < b.built.millis() {
                continue;
            }
            for tile in &aura.affected_tiles {
                if tile.0 as i32 == index.0 && tile.1 as i32 == index.1 {
                    out.push((e.id(), aura.effect as i32))
                }
            }
        }
        out
    }
}
