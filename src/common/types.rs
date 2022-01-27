use ic_cdk::export::candid::{CandidType, Deserialize, Nat};

#[derive(CandidType, Deserialize)]
pub enum Order {
    Market(MarketOrder),
    Limit(LimitOrder),
}

#[derive(CandidType, Deserialize, Clone)]
pub struct MarketOrder {
    pub give_currency: Currency,
    pub take_currency: Currency,
    pub directive: OrderDirective,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct LimitOrder {
    pub when_give_to_take_ratio_reaches: TargetPrice,
    pub execute_market_order: MarketOrder,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum TargetPrice {
    MoreThan(f64),
    LessThan(f64),
}

#[derive(CandidType, Deserialize, Clone)]
pub enum OrderDirective {
    GiveExact(Nat),
    TakeExact(Nat),
}

#[derive(CandidType, Deserialize, Clone)]
pub enum Currency {
    XTC,
    WICP,
}
