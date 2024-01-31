#![no_std]
use gstd::{msg, prelude::*, CodeId};
use tamagotchi_factory_io::*;
pub type EscrowId = u64;

static mut TAMAGOTCHI_FACTORY: Option<TamagotchiFactory> = None;

#[gstd::async_main]
async fn main() {
    let action: FactoryAction = msg::load()
        .expect("Unable to decode `FactoryAction`");
    let factory = unsafe {
        TAMAGOTCHI_FACTORY.get_or_insert(Default::default())
    };
    match action {
        FactoryAction::CreateTamagotchi {
            tamagotchi_name,
        } => factory.create_tamagotchi(tamagotchi_name).await,

        FactoryAction::ConfirmDelivery(tamagotchi_id
        ) => factory.confirm_delivery(tamagotchi_id).await,
    }
}

#[no_mangle]
extern "C" fn init() {
    let tamagotchi_code_id: CodeId = msg::load()
        .expect("Unable to decode CodeId of the Escrow program");
    let tamagotchi_factory = TamagotchiFactory {
        tamagotchi_code_id,
        ..Default::default()
    };
    unsafe { TAMAGOTCHI_FACTORY = Some(tamagotchi_factory) };
}