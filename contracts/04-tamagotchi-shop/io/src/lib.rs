#![no_std]

use codec::{Decode, Encode};
use gmeta::{Metadata, In, InOut, Out};
use gstd::{prelude::*, ActorId, exec, msg};
use store_io::{AttributeId, TransactionId, StoreAction, StoreEvent};
use sharded_fungible_token_io::{FTokenAction, FTokenEvent, LogicAction};
use scale_info::TypeInfo;

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Tamagotchi {
    pub name: String, 
    pub age: u64,
    pub owner: ActorId,
    pub fed: u64,
    pub fed_block: u64,
    pub entertained: u64,
    pub entertained_block: u64,
    pub slept: u64,
    pub slept_block: u64,
    pub approved_account: Option<ActorId>,
    pub ft_contract_id: Option<ActorId>,
    pub transaction_id: u64,
    pub approve_transaction: Option<(TransactionId, ActorId, u128)>
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgAction {
    Name, 
    Age,
    Feed,
    Entertain,
    Sleep,
    Transfer(ActorId),
    Approve(ActorId),
    RevokeApproval,
    SetFTokenContract(ActorId),
    ApproveTokens {
        account: ActorId,
        amount: u128,
    },
    BuyAttribute {
        store_id: ActorId,
        attribute_id: AttributeId,
    },
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgEvent {
    Name(String), 
    Age(u64),
    Fed,
    Entertained,
    Slept,
    Transferred(ActorId),
    Approved(ActorId),
    ApprovalRevoked,
    FTokenContractSet,
    TokensApproved { account: ActorId, amount: u128 },
    ApprovalError,
    AttributeBought(AttributeId),
    CompletePrevPurchase(AttributeId),
    ErrorDuringPurchase,
}

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Handle = InOut<TmgAction, TmgEvent>;
    type State = Out<Tamagotchi>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}

impl Tamagotchi {

    pub fn calculate_hunger(&self) -> u64 {
        HUNGER_PER_BLOCK * ((exec::block_timestamp() - self.fed_block) / 1000)
    }
    
    pub fn calculate_boredom(&self) -> u64 {
        BOREDOM_PER_BLOCK * ((exec::block_timestamp() - self.entertained_block) / 1000)
    }
    
    pub fn calculate_energy(&self) -> u64 {
        ENERGY_PER_BLOCK * ((exec::block_timestamp() - self.slept_block) / 1000)
    }

    pub async fn approve_tokens(&mut self, account: &ActorId, amount: u128) {
        let _result_approve = msg::send_for_reply_as::<_, FTokenEvent>(
            self.ft_contract_id.unwrap(),
            FTokenAction::Message {
                transaction_id: self.transaction_id,
                payload: LogicAction::Approve {
                    approved_account: account.clone(),
                    amount,
                },
            },
            0,
            0,
        )
        .expect("Error in sending a message `FTokenAction::Message`")
        .await;
    }
    
    pub async fn buy_attribute(store: &ActorId, attribute: u32) {
        let _result_buy = msg::send_for_reply_as::<_, StoreEvent>(
            store.clone(),
            StoreAction::BuyAttribute {
                attribute_id: attribute ,
            },
            0,
            0,
        )
        .expect("Error in sending a message `StoreAction::BuyAttribute`")
        .await;
    }

}

pub const HUNGER_PER_BLOCK: u64 = 1;
pub const BOREDOM_PER_BLOCK: u64 = 2;
pub const ENERGY_PER_BLOCK: u64 = 2;

pub const FILL_PER_FEED: u64 = 1000;
pub const FILL_PER_ENTERTAINMENT: u64 = 1000;
pub const FILL_PER_SLEEP: u64 = 1000;

pub const MAX_VALUE: u64 = 10000;