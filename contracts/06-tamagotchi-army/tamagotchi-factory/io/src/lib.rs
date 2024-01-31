#![no_std]

use codec::{Decode, Encode};
//use gmeta::{Metadata, In, InOut, Out};
use gstd::{prelude::*, ActorId, CodeId, collections::BTreeMap, prog::ProgramGenerator, msg};
use scale_info::TypeInfo;

pub type TamagotchiId = u64;

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TamagotchiFactory {
    pub tamagotchi_number: TamagotchiId,
    pub id_to_address: BTreeMap<TamagotchiId, ActorId>,
    pub tamagotchi_code_id: CodeId,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FactoryAction {
    CreateTamagotchi {
        tamagotchi_name: String,
    },
    ConfirmDelivery(TamagotchiId),
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FactoryEvent {
    TamagotchiCreated {
        tamagotchi_id: TamagotchiId,
        tamagotchi_address: ActorId,
    },
    DeliveryConfirmed(ActorId),
}

/* pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Handle = InOut<FactoryAction, FactoryEvent>;
    type State = Out<TamagotchiFactory>;
    type Reply = ();
    type Others = ();
    type Signal = ();
} */

impl TamagotchiFactory {
    pub async fn create_tamagotchi(&mut self, tamagotchi_name: String) {
        let (address, _) = ProgramGenerator::create_program_with_gas_for_reply(
            self.tamagotchi_code_id, tamagotchi_name
            .encode(),
            GAS_FOR_CREATION,
            0,
            0,
        )
        .expect("Error during Escrow program initialization")
        .await
        .expect("Program was not initialized");
        self.tamagotchi_number = self.tamagotchi_number.saturating_add(1);
        self.id_to_address.insert(self.tamagotchi_number, address);
        msg::reply(
            FactoryEvent::TamagotchiCreated {
                tamagotchi_id: self.tamagotchi_number,
                tamagotchi_address: address,
            },
            0,
        )
        .expect("Error during a reply `FactoryEvent::ProgramCreated`");
    }

    pub async fn confirm_delivery(&self, tamagotchi_id: TamagotchiId) {
        let tamagotchi_address = self.get_tamagotchi_address(tamagotchi_id);
        msg::reply(FactoryEvent::DeliveryConfirmed(tamagotchi_address), 0)
            .expect("Error during a reply `FactoryEvent::DeliveryConfirmed`");
    }
    
    fn get_tamagotchi_address(&self, tamagotchi_id: TamagotchiId) -> ActorId {
        *self
            .id_to_address
            .get(&tamagotchi_id)
            .expect("The tamagotchi with indicated id does not exist")
    }
    
}

pub static GAS_FOR_CREATION: u64 = 2;