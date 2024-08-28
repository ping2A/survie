use bevy::prelude::*;

use crate::models::configurations::coin_config::CoinConfig;

#[derive(Default, Resource)]
pub struct ItemConfigHandles {
    pub coin: CoinConfig,
}

pub fn preload_item_system(
    mut item_config_handles: ResMut<ItemConfigHandles>,
) 
{
    info!("preload_item_system");
}