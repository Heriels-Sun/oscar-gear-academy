#![no_std]
use gstd::{msg, prelude::*, exec};
use tamagotchi_io::*;

static mut TAMAGOTCHI: Option<Tamagotchi> = None;



#[no_mangle]
extern fn init() {
    let name: String = msg::load().expect("Failed to decode Tamagotchi name");
    let current_time_block = exec::block_timestamp();

    let tmg = Tamagotchi {
        name, 
        age: current_time_block,
        owner: msg::source(),
        fed: MAX_VALUE,
        fed_block: current_time_block,
        entertained: MAX_VALUE,
        entertained_block: current_time_block,
        slept: MAX_VALUE,
        slept_block: current_time_block,
        approved_account: None,
        ft_contract_id: Default::default(),
        transaction_id: Default::default(),
        approve_transaction: None,
    };

    unsafe {
        TAMAGOTCHI = Some(tmg);
    }
}

#[no_mangle]
extern "C" fn handle() {
    let action: TmgAction = msg::load().expect("Unable to decode `TmgAction`");
    let source   = msg::source();
    let tmg = unsafe { TAMAGOTCHI.get_or_insert(Default::default()) };
    match action {
        TmgAction::Name => {
            msg::reply(TmgEvent::Name(tmg.name.clone()), 0)
                .expect("Error in a reply `TmgEvent::Name`");
        }
        TmgAction::Age => {
            let age = exec::block_timestamp() - tmg.age;
            msg::reply(TmgEvent::Age(age), 0)
                .expect("Error in a reply `TmgEvent::Age`");
        }
        TmgAction::Feed => {
            tmg.fed += FILL_PER_FEED - tmg.calculate_hunger();
            tmg.fed_block = exec::block_timestamp();
            tmg.fed = if tmg.fed > MAX_VALUE {
                MAX_VALUE
            } else {
                tmg.fed
            };
        msg::reply(TmgEvent::Fed, 0).expect("Error in a reply `TmgEvent::Fed`");

        },
        TmgAction::Entertain => {
            tmg.fed += FILL_PER_ENTERTAINMENT - tmg.calculate_boredom();
            tmg.fed_block = exec::block_timestamp();
            tmg.fed = if tmg.fed > MAX_VALUE {
                MAX_VALUE
            } else {
                tmg.fed
            };
        msg::reply(TmgEvent::Entertained, 0).expect("Error in a reply `TmgEvent::Entertained`");

        },
        TmgAction::Sleep => {
            tmg.fed += FILL_PER_FEED - tmg.calculate_energy();
            tmg.fed_block = exec::block_timestamp();
            tmg.fed = if tmg.fed > MAX_VALUE {
                MAX_VALUE
            } else {
                tmg.fed
            };
        msg::reply(TmgEvent::Slept, 0).expect("Error in a reply `TmgEvent::Slept`");
        },
        TmgAction::Transfer(new_actor) => {
            if source == tmg.owner || source == tmg.approved_account.unwrap() {
                tmg.owner = new_actor;
            }
            msg::reply(TmgEvent::Transferred(tmg.owner), 0).expect("Error in a reply `TmgEvent::Transferred`");
        },
        TmgAction::Approve(new_actor) => {
            if source == tmg.owner {
                tmg.approved_account = Some(new_actor);
            }
            msg::reply(TmgEvent::Approved(tmg.approved_account.unwrap()), 0).expect("Error in a reply `TmgEvent::Approved`");
        },
        TmgAction::RevokeApproval => {
            tmg.approved_account = None;
            msg::reply(TmgEvent::ApprovalRevoked, 0).expect("Error in a reply `TmgEvent::ApprovalRevoked`");
        },
        TmgAction::SetFTokenContract(contract) => {
            tmg.ft_contract_id = Some(contract);
            msg::reply(TmgEvent::FTokenContractSet, 0).expect("Error in a reply `TmgEvent::FTokenContractSet`");
        },
        TmgAction::ApproveTokens { account, amount } => {
            Tamagotchi::approve_tokens(tmg, &account, amount).await;
            msg::reply(TmgEvent::TokensApproved { account, amount }, 0).expect("Error in a reply `TmgEvent::TokensApproved`");
        },
        TmgAction::BuyAttribute { store_id, attribute_id } => {
            Tamagotchi::buy_attribute(&store_id, attribute_id).await;
            msg::reply(TmgEvent::AttributeBought(attribute_id), 0).expect("Error in a reply `TmgEvent::TokensApproved`");
        },

    };
}

#[no_mangle]
extern fn state() {
    let tmg = unsafe { TAMAGOTCHI.take().expect("Error in taking current state") };
    msg::reply(tmg, 0).expect("Failed to reply state");
}