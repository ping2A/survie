use bevy::prelude::*;

use crate::assets_handling::preload_item_system::ItemConfigHandles;
use crate::assets_handling::preload_projectile_system::ProjectileConfigHandles;

#[derive(Default, Resource)]
pub struct TextureHandles {
    pub font: Handle<Font>,
    pub coin_sprite: Handle<Image>,
    pub hot_dog_sprite: Handle<Image>,
    pub barrel_sprite: Handle<Image>,
    pub basic_projectile_fireball: Handle<TextureAtlas>,
    pub projectile_fireball: Handle<TextureAtlas>,
    pub background_tile: Handle<TextureAtlas>,
    pub sold_button: Handle<Image>,
    pub turret_unit: Handle<Image>,
    pub slime_unit: Handle<Image>,
    pub death_ball_unit: Handle<TextureAtlas>,
    pub psy_rock_unit: Handle<Image>,
    pub radiation: Handle<Image>,
    pub shield: Handle<Image>,
    pub sword: Handle<Image>,
    pub death_counter: Handle<Image>,
    pub highscore: Handle<Image>,
}

pub fn preload_texture_system(
    asset_server: Res<AssetServer>,
    item_handles: Res<ItemConfigHandles>,
    projectile_handles: Res<ProjectileConfigHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut texture_handles: ResMut<TextureHandles>,
) 
{
    info!("preload_texture_system");

/* 
    texture_handles.hot_dog_sprite = asset_server.load(&item_handles.hot_dog.sprite_path);
    texture_handles.barrel_sprite = asset_server.load(&item_handles.barrel.sprite_path);

    texture_handles.sold_button = asset_server.load("sprites/sold_sign.png");

    texture_handles.turret_unit = asset_server.load("sprites/unit_sprites/turret_unit.png");
    texture_handles.slime_unit = asset_server.load("sprites/unit_sprites/slime.png");
    texture_handles.death_ball_unit = asset_server.load("sprites/unit_sprites/death_ball.png");
    texture_handles.psy_rock_unit = asset_server.load("sprites/unit_sprites/psy_rock.png");
    texture_handles.radiation = asset_server.load("sprites/unit_sprites/radiation.png");
    texture_handles.shield = asset_server.load("sprites/unit_sprites/shield.png");
    texture_handles.sword = asset_server.load("sprites/sword.png");
*/
    texture_handles.turret_unit = asset_server.load("default/sprites/turret.png");
    texture_handles.psy_rock_unit = asset_server.load("default/sprites/weapons/fireball.png");
    texture_handles.sword = asset_server.load("default/sprites/weapons/sword2.png");
    texture_handles.shield = asset_server.load("default/sprites/items/shield.png");
    texture_handles.radiation = asset_server.load("default/sprites/weapons/radiation.png");



    texture_handles.coin_sprite = asset_server.load(&item_handles.coin.sprite_path);

    texture_handles.basic_projectile_fireball = texture_atlases.add(TextureAtlas::from_grid(asset_server.load("default/sprites/weapons/basic_fireball.png"),  
        Vec2::new(32.0, 32.0), 
            5, 1, None, None));

    texture_handles.projectile_fireball = texture_atlases.add(TextureAtlas::from_grid(asset_server.load(&projectile_handles.basic_projectile.sprite_path),  
    Vec2::new(64.0, 32.0), 
        5, 1, None, None));

    texture_handles.death_ball_unit = texture_atlases.add(TextureAtlas::from_grid(asset_server.load("default/sprites/weapons/deathball.png"),  
        Vec2::new(32.0, 32.0), 
            5, 1, None, None));

    texture_handles.background_tile = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("default/sprites/world/background.png"), 
        Vec2::new(32.0, 32.0), 
        15, 5, None, None));

    texture_handles.death_counter = asset_server.load("default/sprites/items/death_counter.png");
    texture_handles.highscore = asset_server.load("default/sprites/items/highscore.png");

    texture_handles.font = asset_server.load("default/fonts/PressStart2P.ttf");
}