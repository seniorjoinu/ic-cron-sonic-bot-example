mod clients;

use ic_cdk::export::candid::{export_service, CandidType, Deserialize, Principal};
use ic_cdk::storage::{stable_restore, stable_save};
use ic_cdk::trap;
use ic_cdk_macros::{init, pre_upgrade, query};
use ic_cron::implement_cron;

// -------------------- HELPERS -------------------

pub enum OrderType {
    Buy,
    Sell,
}

pub enum Currency {
    XTC,
    WICP,
}

// -------------------- STATE ---------------------

#[init]
pub fn init(controller: Principal) {
    unsafe {
        STATE = Some(State {
            xtc_canister: Principal::from_text("aanaa-xaaaa-aaaah-aaeiq-cai").unwrap(),
            wicp_canister: Principal::from_text("utozz-siaaa-aaaam-qaaxq-cai").unwrap(),
            sonic_swap_canister: Principal::from_text("3xwpq-ziaaa-aaaah-qcn4a-cai").unwrap(),
            controller,
        })
    }
}

#[derive(CandidType, Deserialize)]
struct State {
    pub xtc_canister: Principal,
    pub wicp_canister: Principal,
    pub sonic_swap_canister: Principal,
    pub controller: Principal,
}

static mut STATE: Option<State> = None;

pub fn get_state() -> &'static mut State {
    unsafe {
        match STATE.as_mut() {
            None => {
                let (state,) = stable_restore().unwrap();
                STATE = Some(state);

                get_state()
            }
            Some(s) => s,
        }
    }
}

#[pre_upgrade]
pub fn pre_upgrade_hook() {
    unsafe { stable_save((STATE.unwrap(),)).unwrap() }
}

implement_cron!();

// ---------------- CANDID -----------------------

export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
