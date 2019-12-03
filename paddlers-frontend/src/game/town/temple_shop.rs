use paddlers_shared_lib::api::shop::ProphetPurchase;
use crate::prelude::*;
use crate::game::{components::UiMenu, game_event_manager::*, player_info::PlayerInfo};
use crate::gui::sprites::{SingleSprite, SpriteSet};
use crate::net::game_master_api::RestApiState;
use crate::net::state::current_village;

pub fn new_temple_menu(player_info: &PlayerInfo) -> UiMenu {
    UiMenu::new_shop_menu().with_shop_item(
        GameEvent::HttpBuyProphet,
        SpriteSet::Simple(SingleSprite::Prophet),
        player_info.prophet_price(),
    )
}

pub fn purchase_prophet(rest: &mut RestApiState, player_info: &PlayerInfo) -> PadlResult<()> {
    if player_info.prophets_limit() <= player_info.prophets_total() {
        return PadlErrorCode::NotEnoughKarma.usr();
    }
    rest.http_buy_prophet(ProphetPurchase {
        village: current_village(),
    })?;
    Ok(())
}
