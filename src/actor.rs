mod clients;
mod common;

use crate::clients::dip20::Dip20;
use crate::clients::sonic::Sonic;
use crate::clients::xtc::{XTCBurnPayload, XTC};
use crate::common::guards::controller_guard;
use crate::common::types::{Currency, LimitOrder, MarketOrder, Order, OrderDirective, TargetPrice};
use crate::common::utils::UnwrapOrTrap;
use bigdecimal::num_bigint::{BigInt, ToBigInt};
use bigdecimal::num_traits::Pow;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use ic_cdk::api::{canister_balance, time};
use ic_cdk::export::candid::{export_service, CandidType, Deserialize, Int, Nat, Principal};
use ic_cdk::id;
use ic_cdk::storage::{stable_restore, stable_save};
use ic_cdk_macros::{heartbeat, init, pre_upgrade, query, update};
use ic_cron::implement_cron;
use ic_cron::types::{Iterations, SchedulingInterval, TaskId};

#[update(guard = controller_guard)]
pub async fn deposit(currency: Currency, amount: Nat) {
    let state = *get_state();
    let token = token_id_by_currency(currency);

    Dip20::approve(&token, state.sonic_swap_canister, amount.clone())
        .await
        .unwrap_or_trap("Unable to approve tokens: call failed")
        .0
        .unwrap_or_trap("Unable to approve tokens: internal error");

    Sonic::deposit(&state.sonic_swap_canister, token, amount)
        .await
        .unwrap_or_trap("Unable to deposit tokens: call failed")
        .0
        .to_res()
        .unwrap_or_trap("Unable to deposit tokens: internal error");
}

#[update(guard = controller_guard)]
pub async fn withdraw(currency: Currency, amount: Nat) {
    let state = *get_state();
    let token = token_id_by_currency(currency);

    Sonic::withdraw(&state.sonic_swap_canister, token, amount)
        .await
        .unwrap_or_trap("Unable to withdraw tokens: call failed")
        .0
        .to_res()
        .unwrap_or_trap("Unable to withdraw tokens: internal error");
}

#[update(guard = controller_guard)]
pub async fn transfer(currency: Currency, to: Principal, amount: Nat) {
    let state = get_state();
    let token = token_id_by_currency(currency);

    Dip20::transfer(&token, to, amount)
        .await
        .unwrap_or_trap("Unable to transfer tokens: call failed")
        .0
        .unwrap_or_trap("Unable to transfer tokens: internal error");
}

#[update(guard = controller_guard)]
pub async fn mint_xtc_with_own_cycles(amount: u64) {
    let state = get_state();

    XTC::mint(&state.xtc_canister, id(), amount)
        .await
        .unwrap_or_trap("Unable to mint XTC with cycles: call failed")
        .0
        .unwrap_or_trap("Unable to mint XTC with cycles: internal error");
}

#[update(guard = controller_guard)]
pub async fn burn_xtc_for_own_cycles(amount: u64) {
    let state = get_state();
    let payload = XTCBurnPayload {
        canister_id: id(),
        amount,
    };

    XTC::burn(&state.xtc_canister, payload)
        .await
        .unwrap_or_trap("Unable to burn XTC for cycles: call failed")
        .0
        .unwrap_or_trap("Unable to burn XTC for cycles: internal error");
}

#[update]
pub async fn my_token_balance(currency: Currency) -> Nat {
    let state = get_state();
    let token = token_id_by_currency(currency);

    let (balance,) = Dip20::balance_of(&token, id())
        .await
        .unwrap_or_trap("Unable to fetch my balance at token");

    balance
}

#[update]
pub async fn my_sonic_balance(currency: Currency) -> Nat {
    let state = get_state();
    let token = token_id_by_currency(currency);

    let (balance,) = Sonic::balance_of(&state.sonic_swap_canister, token.to_text(), id())
        .await
        .unwrap_or_trap("Unable to fetch my balance at Sonic");

    balance
}

#[query]
pub fn my_cycles_balance() -> u64 {
    canister_balance()
}

async fn get_swap_price_internal(give_currency: Currency, take_currency: Currency) -> BigDecimal {
    let state = *get_state();
    let give_token = token_id_by_currency(give_currency);
    let take_token = token_id_by_currency(take_currency);

    let (pair_opt,) = Sonic::get_pair(&state.sonic_swap_canister, give_token, take_token)
        .await
        .unwrap_or_trap("Unable to fetch pair at Sonic");

    let pair = pair_opt.unwrap();

    let give_reserve = BigDecimal::from(pair.reserve0.0.to_bigint().unwrap());
    let take_reserve = BigDecimal::from(pair.reserve1.0.to_bigint().unwrap());

    give_reserve / take_reserve
}

#[update]
pub async fn get_swap_price(give_currency: Currency, take_currency: Currency) -> f64 {
    let give_token = token_id_by_currency(give_currency.clone());
    let take_token = token_id_by_currency(take_currency.clone());

    let price_bd = get_swap_price_internal(give_currency, take_currency).await;

    let (give_token_decimals,) = Dip20::decimals(&give_token)
        .await
        .unwrap_or_trap("Unable to fetch give_token decimals");

    let (take_token_decimals,) = Dip20::decimals(&take_token)
        .await
        .unwrap_or_trap("Unable to fetch take_token decimals");

    let decimals_dif =
        give_token_decimals.to_i32().unwrap() - take_token_decimals.to_i32().unwrap();

    let decimals_modifier = 10f64.pow(decimals_dif);

    price_bd.to_f64().unwrap() * decimals_modifier
}

fn token_id_by_currency(currency: Currency) -> Principal {
    let state = get_state();

    match currency {
        Currency::XTC => state.xtc_canister,
        Currency::WICP => state.wicp_canister,
    }
}

#[update(guard = controller_guard)]
pub async fn add_order(order: Order) -> Option<TaskId> {
    match order {
        Order::Market(market_order) => {
            execute_market_order(market_order).await;

            None
        }
        Order::Limit(limit_order) => {
            // we need to somehow freeze tokens spent for limit orders

            let task_id = cron_enqueue(
                limit_order,
                SchedulingInterval {
                    delay_nano: 0,
                    interval_nano: 1_000_000_000 * 10, // check each 30 seconds,
                    iterations: Iterations::Exact(1),
                },
            )
            .unwrap_or_trap("Unable to schedule a task");

            Some(task_id)
        }
    }
}

#[heartbeat]
pub fn tick() {
    for task in cron_ready_tasks() {
        let limit_order = task
            .get_payload::<LimitOrder>()
            .expect("Unable to parse limit order");

        ic_cdk::block_on(async {
            let res = execute_limit_order(limit_order.clone()).await;

            if !res {
                cron_enqueue(
                    limit_order,
                    SchedulingInterval {
                        delay_nano: 0,
                        interval_nano: 1_000_000_000 * 10,
                        iterations: Iterations::Exact(1),
                    },
                )
                .unwrap_or_trap("Unable to reschedule a task");
            }
        });
    }
}

async fn execute_limit_order(order: LimitOrder) -> bool {
    let price = get_swap_price(
        order.market_order.give_currency.clone(),
        order.market_order.take_currency.clone(),
    )
    .await;

    match order.target_price_condition {
        TargetPrice::MoreThan(target_price) => {
            if price >= target_price {
                execute_market_order(order.market_order).await;
                true
            } else {
                false
            }
        }
        TargetPrice::LessThan(target_price) => {
            if price <= target_price {
                execute_market_order(order.market_order).await;
                true
            } else {
                false
            }
        }
    }
}

async fn execute_market_order(order: MarketOrder) -> Nat {
    let state = *get_state();

    let give_token = token_id_by_currency(order.give_currency.clone());
    let take_token = token_id_by_currency(order.take_currency.clone());

    let slippage_bd = BigDecimal::from_f64(0.99f64).unwrap(); // can tolerate 1% slippage
    let deadline = Int(BigInt::from(time() + 1_000_000_000 * 20)); // 20 seconds til now
    let this = id();

    let price_bd = get_swap_price_internal(order.give_currency, order.take_currency).await;

    match order.directive {
        OrderDirective::GiveExact(give_amount) => {
            let give_amount_bd = BigDecimal::from(give_amount.0.to_bigint().unwrap());

            let take_amount_min_bd = give_amount_bd / price_bd * slippage_bd;

            let take_amount_min = Nat(take_amount_min_bd
                .to_bigint()
                .unwrap()
                .to_biguint()
                .unwrap());

            Sonic::swap_exact_tokens_for_tokens(
                &state.sonic_swap_canister,
                give_amount,
                take_amount_min,
                vec![give_token.to_text(), take_token.to_text()],
                this,
                deadline,
            )
            .await
            .unwrap_or_trap("Unable to swap exact tokens: call failed")
            .0
            .to_res()
            .unwrap_or_trap("Unable to swap exact tokens: internal error")
        }
        OrderDirective::TakeExact(take_amount) => {
            let take_amount_bd = BigDecimal::from(take_amount.0.to_bigint().unwrap());

            let give_amount_max_bd = take_amount_bd * price_bd * slippage_bd;
            let give_amount_max = Nat(give_amount_max_bd
                .to_bigint()
                .unwrap()
                .to_biguint()
                .unwrap());

            Sonic::swap_tokens_for_exact_tokens(
                &state.sonic_swap_canister,
                take_amount,
                give_amount_max,
                vec![give_token.to_text(), take_token.to_text()],
                this,
                deadline,
            )
            .await
            .unwrap_or_trap("Unable to swap to exact tokens: call failed")
            .0
            .to_res()
            .unwrap_or_trap("Unable to swap exact tokens: internal error")
        }
    }
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

#[derive(CandidType, Deserialize, Clone, Copy)]
pub struct State {
    pub xtc_canister: Principal,
    pub wicp_canister: Principal,
    pub sonic_swap_canister: Principal,
    pub controller: Principal,
}

pub static mut STATE: Option<State> = None;

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
