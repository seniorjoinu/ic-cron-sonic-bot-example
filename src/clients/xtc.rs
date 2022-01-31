use crate::clients::dip20::Dip20TxReceipt;
use crate::Nat;
use async_trait::async_trait;
use ic_cdk::api::call::{call_with_payment, CallResult};
use ic_cdk::call;
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize)]
pub struct XTCBurnPayload {
    pub canister_id: Principal,
    pub amount: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum XTCBurnError {
    InsufficientBalance,
    InvalidTokenContract,
    NotSufficientLiquidity,
}

pub type XTCBurnResult = Result<u64, XTCBurnError>;

#[async_trait]
pub trait XTC {
    async fn mint(&self, to: Principal, cycles: u64) -> CallResult<(Dip20TxReceipt,)>;
    async fn burn(&self, payload: XTCBurnPayload) -> CallResult<(XTCBurnResult,)>;
}

#[async_trait]
impl XTC for Principal {
    async fn mint(&self, to: Principal, cycles: u64) -> CallResult<(Dip20TxReceipt,)> {
        call_with_payment(*self, "mint", (to, Nat::from(0)), cycles).await
    }

    async fn burn(&self, payload: XTCBurnPayload) -> CallResult<(XTCBurnResult,)> {
        call(*self, "burn", (payload,)).await
    }
}
