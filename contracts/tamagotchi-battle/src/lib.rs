#![no_std]
use gstd::{msg, prelude::*, collections::BTreeMap};
use tamagotchi_battle_io::*;

static mut TAMAGOTCHI_BATTLE: Option<Battle> = None;

#[no_mangle]
extern "C" fn init() {
    let BattleInit { tmg_store_id } = msg::load()
        .expect("Unable to decode CodeId of the Escrow program");
    let tamagotchi_battle = Battle {
        tmg_store_id,
        weapons_data: BTreeMap::from([
            (SWORD_NORMAL_ID, SWORD_NORMAL_POWER),
            (SWORD_RARE_ID, SWORD_RARE_POWER),
            (SWORD_MYTHICAL_ID, SWORD_MYTHICAL_POWER),
            (SWORD_LEGENDARY_ID, SWORD_LEGENDARY_POWER),
            (BOW_NORMAL_ID, BOW_NORMAL_POWER),
            (BOW_RARE_ID, BOW_RARE_POWER),
            (BOW_MYTHICAL_ID, BOW_MYTHICAL_POWER),
            (BOW_LEGENDARY_ID, BOW_LEGENDARY_POWER)
        ]),
        shields_data: BTreeMap::from([
            (SHIELD_NORMAL_ID, SHIELD_NORMAL_PROTECTION),
            (SHIELD_RARE_ID, SHIELD_RARE_PROTECTION),
            (SHIELD_MYTHICAL_ID, SHIELD_MYTHICAL_PROTECTION),
            (SHIELD_LEGENDARY_ID, SHIELD_LEGENDARY_PROTECTION)
        ]),
        ..Default::default()
    };
    unsafe { TAMAGOTCHI_BATTLE = Some(tamagotchi_battle) };
}   

#[gstd::async_main]
async fn main() {
    let action: BattleAction = msg::load()
        .expect("Unable to decode `FactoryAction`");
    let tmg_battle = unsafe {
        TAMAGOTCHI_BATTLE.get_or_insert(Default::default())
    };
    
    match action {
        BattleAction::Register {
            tamagotchi_id,
        } => {
            tmg_battle.register(&tamagotchi_id).await;            
        },
        BattleAction::Move(direction, attribute) => {
            tmg_battle.make_move(direction, attribute);
        },
        BattleAction::UpdateInfo => {
            tmg_battle.update_info().await;
        },
        BattleAction::StartNewGame => {
            tmg_battle.reset_contract();
        },
        BattleAction::ReserveGas { 
            reservation_amount, 
            duration 
        } => {
            tmg_battle.make_reservation(reservation_amount, duration);
        }
    }
}

#[no_mangle]
extern fn state() {
    msg::reply(state_ref(), 0)
        .expect("Failed to share state");
}

fn state_ref() -> &'static Battle {
    let state = unsafe { TAMAGOTCHI_BATTLE.as_ref() };
    unsafe { state.unwrap_unchecked() }
}