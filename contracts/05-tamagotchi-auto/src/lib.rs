#![no_std]
use gstd::{msg, prelude::*, exec, ReservationId};
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
        reservations: Default::default(),
    };

    unsafe {
        TAMAGOTCHI = Some(tmg);
    }

    msg::send_for_reply(exec::program_id(), TmgAction::CheckState, 0, 0)
        .expect("Unable to send the first check state");
}

//#[no_mangle]
//extern "C" fn handle() {
#[gstd::async_main]
async fn main() {
    let action: TmgAction = msg::load().expect("Unable to decode `TmgAction`");
    let source   = msg::source();
    let tmg = unsafe { TAMAGOTCHI.get_or_insert(Default::default()) };
    match action {
        TmgAction::Name => {
            let _reserve = exec::system_reserve_gas(100);
            msg::reply(TmgEvent::Name(tmg.name.clone()), 0)
                .expect("Error in a reply `TmgEvent::Name`");
        }
        TmgAction::Age => {
             let _reserve = exec::system_reserve_gas(100);
            let age = exec::block_timestamp() - tmg.age;
            msg::reply(TmgEvent::Age(age), 0)
                .expect("Error in a reply `TmgEvent::Age`");
        }
        TmgAction::Feed => {
             let _reserve = exec::system_reserve_gas(100);
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
             let _reserve = exec::system_reserve_gas(100);
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
             let _reserve = exec::system_reserve_gas(100);
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
             let _reserve = exec::system_reserve_gas(100);
            if source == tmg.owner || source == tmg.approved_account.unwrap() {
                tmg.owner = new_actor;
            }
            msg::reply(TmgEvent::Transferred(tmg.owner), 0).expect("Error in a reply `TmgEvent::Transferred`");
        },
        TmgAction::Approve(new_actor) => {
             let _reserve = exec::system_reserve_gas(100);
            if source == tmg.owner {
                tmg.approved_account = Some(new_actor);
            }
            msg::reply(TmgEvent::Approved(tmg.approved_account.unwrap()), 0).expect("Error in a reply `TmgEvent::Approved`");
        },
        TmgAction::RevokeApproval => {
             let _reserve = exec::system_reserve_gas(100);
            tmg.approved_account = None;
            msg::reply(TmgEvent::ApprovalRevoked, 0).expect("Error in a reply `TmgEvent::ApprovalRevoked`");
        },
        TmgAction::SetFTokenContract(contract) => {
            tmg.ft_contract_id = Some(contract);
            msg::reply(TmgEvent::FTokenContractSet, 0).expect("Error in a reply `TmgEvent::FTokenContractSet`");
        },
        TmgAction::ApproveTokens { account, amount } => {
            //Tamagotchi::approve_tokens(tmg, &account, amount);
            tmg.approve_tokens(&account, amount).await;
            msg::reply(TmgEvent::TokensApproved { account, amount }, 0).expect("Error in a reply `TmgEvent::TokensApproved`");
        },
        TmgAction::BuyAttribute { store_id, attribute_id } => {
            tmg.buy_attribute(&store_id, attribute_id).await;
            msg::reply(TmgEvent::AttributeBought(attribute_id), 0).expect("Error in a reply `TmgEvent::TokensApproved`");
        },
        TmgAction::CheckState => {
            let last_reservation: Option<ReservationId> = tmg.reservations.last().copied();
            match last_reservation {
                Some(last) => {
                    let needs_something: Option<TmgEvent> = tmg.tmg_needs_something();
                    match needs_something {
                        Some(event) => {
                            msg::send_from_reservation(last, tmg.owner, event, 0).expect("There is an error sending a message to the auto Tamagotchi");
                        }
                        None => {}
                    }
                    msg::send_delayed_from_reservation(last, exec::program_id(), TmgAction::CheckState, 0, 500)
                        .expect("Unnable to send, not enough gas");
                }
                None => {
                    msg::reply(TmgEvent::MakeReservation, 0).expect("There is not enough gas for delayed messages `TmgEvent::MakeReservation`");
                }
            }
            
        },
        TmgAction::ReserveGas { reservation_amount, duration } => {
            tmg.make_reservation(reservation_amount, duration);
            msg::reply(TmgEvent::GasReserved, 0).expect("Error in a reply `TmgEvent::TokensApproved`");
        },

    };
}

#[no_mangle]
extern fn state() {
    let tmg = unsafe { TAMAGOTCHI.take().expect("Error in taking current state") };
    msg::reply(tmg, 0).expect("Failed to reply state");
}