use async_trait::async_trait;
use ic_cdk::api::call::{call_with_payment, CallResult};
use ic_cdk::call;
use ic_cdk::export::candid::{CandidType, Deserialize, Nat, Principal};

#[derive(CandidType, Deserialize)]
pub struct Dip20Metadata {
    pub fee: Nat,
    pub decimals: u8,
    pub owner: Principal,
    pub logo: String,
    pub name: String,
    pub totalSupply: Nat,
    pub symbol: String,
}

#[derive(CandidType, Deserialize)]
pub struct Dip20TokenInfo {
    pub holderNumber: u64,
    pub deployTime: u8,
    pub metadata: Dip20Metadata,
    pub historySize: u64,
    pub cycles: u64,
    pub feeTo: Principal,
}

#[derive(CandidType, Deserialize)]
pub enum Dip20TxError {
    InsufficientAllowance,
    InsufficientBalance,
    ErrorOperationStyle,
    Unauthorized,
    LedgerTrap,
    ErrorTo,
    Other,
    BlockUsed,
    AmountTooSmall,
}

pub type Dip20TxReceipt = Result<Nat, Dip20TxError>;

#[async_trait]
pub trait Dip20 {
    async fn transfer(&self, to: Principal, value: Nat) -> CallResult<(Dip20TxReceipt,)>;
    async fn transfer_from(
        &self,
        from: Principal,
        to: Principal,
        value: Nat,
    ) -> CallResult<(Dip20TxReceipt,)>;
    async fn approve(&self, spender: Principal, value: Nat) -> CallResult<(Dip20TxReceipt,)>;
    async fn mint(&self, to: Principal, amount: Nat, cycles: u64) -> CallResult<(Dip20TxReceipt,)>;
    async fn burn(&self, to: Principal, amount: Nat) -> CallResult<(Dip20TxReceipt,)>;

    async fn set_name(&self, name: String) -> CallResult<()>;
    async fn name(&self) -> CallResult<(String,)>;

    async fn set_logo(&self, logo: String) -> CallResult<()>;
    async fn get_logo(&self) -> CallResult<(String,)>;

    async fn set_fee(&self, fee: Nat) -> CallResult<()>;
    async fn set_fee_to(&self, fee_to: Nat) -> CallResult<()>;

    async fn set_owner(&self, owner: Principal) -> CallResult<()>;
    async fn owner(&self) -> CallResult<(Principal,)>;

    async fn symbol(&self) -> CallResult<(String,)>;
    async fn decimals(&self) -> CallResult<(u8,)>;
    async fn total_supply(&self) -> CallResult<(Nat,)>;
    async fn balance_of(&self, id: Principal) -> CallResult<(Nat,)>;
    async fn allowance(&self, owner: Principal, spender: Principal) -> CallResult<(Nat,)>;
    async fn get_metadata(&self) -> CallResult<(Dip20Metadata,)>;
    async fn history_size(&self) -> CallResult<(usize,)>;
    async fn get_token_info(&self) -> CallResult<(Dip20TokenInfo,)>;
    async fn get_holders(&self, start: usize, limit: usize)
        -> CallResult<(Vec<(Principal, Nat)>,)>;
    async fn get_allowance_size(&self) -> CallResult<(usize,)>;
    async fn get_user_approvals(&self, who: Principal) -> CallResult<(Vec<(Principal, Nat)>,)>;
}

#[async_trait]
impl Dip20 for Principal {
    async fn transfer(&self, to: Principal, value: Nat) -> CallResult<(Dip20TxReceipt,)> {
        call(*self, "transfer", (to, value)).await
    }

    async fn transfer_from(
        &self,
        from: Principal,
        to: Principal,
        value: Nat,
    ) -> CallResult<(Dip20TxReceipt,)> {
        call(*self, "transferFrom", (from, to, value)).await
    }

    async fn approve(&self, spender: Principal, value: Nat) -> CallResult<(Dip20TxReceipt,)> {
        call(*self, "approve", (spender, value)).await
    }

    async fn mint(&self, to: Principal, amount: Nat, cycles: u64) -> CallResult<(Dip20TxReceipt,)> {
        call_with_payment(*self, "mint", (to, amount), cycles).await
    }

    async fn burn(&self, to: Principal, amount: Nat) -> CallResult<(Dip20TxReceipt,)> {
        call(*self, "burn", (to, amount)).await
    }

    async fn set_name(&self, name: String) -> CallResult<()> {
        call(*self, "setName", (name)).await
    }

    async fn name(&self) -> CallResult<(String,)> {
        call(*self, "name", ()).await
    }

    async fn set_logo(&self, logo: String) -> CallResult<()> {
        call(*self, "setLogo", (logo,)).await
    }

    async fn get_logo(&self) -> CallResult<(String,)> {
        call(*self, "getLogo", ()).await
    }

    async fn set_fee(&self, fee: Nat) -> CallResult<()> {
        call(*self, "setFee", (fee,)).await
    }

    async fn set_fee_to(&self, fee_to: Nat) -> CallResult<()> {
        call(*self, "setFeeTo", (fee_to,)).await
    }

    async fn set_owner(&self, owner: Principal) -> CallResult<()> {
        call(*self, "setOwner", (owner,)).await
    }

    async fn owner(&self) -> CallResult<(Principal,)> {
        call(*self, "owner", ()).await
    }

    async fn symbol(&self) -> CallResult<(String,)> {
        call(*self, "symbol", ()).await
    }

    async fn decimals(&self) -> CallResult<(u8,)> {
        call(*self, "decimals", ()).await
    }

    async fn total_supply(&self) -> CallResult<(Nat,)> {
        call(*self, "totalSupply", ()).await
    }

    async fn balance_of(&self, id: Principal) -> CallResult<(Nat,)> {
        call(*self, "balanceOf", (id,)).await
    }

    async fn allowance(&self, owner: Principal, spender: Principal) -> CallResult<(Nat,)> {
        call(*self, "allowance", (owner, spender)).await
    }

    async fn get_metadata(&self) -> CallResult<(Dip20Metadata,)> {
        call(*self, "getMetadata", ()).await
    }

    async fn history_size(&self) -> CallResult<(usize,)> {
        call(*self, "historySize", ()).await
    }

    async fn get_token_info(&self) -> CallResult<(Dip20TokenInfo,)> {
        call(*self, "getTokenInfo", ()).await
    }

    async fn get_holders(
        &self,
        start: usize,
        limit: usize,
    ) -> CallResult<(Vec<(Principal, Nat)>,)> {
        call(*self, "getHolders", (start, limit)).await
    }

    async fn get_allowance_size(&self) -> CallResult<(usize,)> {
        call(*self, "getAllowanceSize", ()).await
    }

    async fn get_user_approvals(&self, who: Principal) -> CallResult<(Vec<(Principal, Nat)>,)> {
        call(*self, "getUserApprovals", (who,)).await
    }
}
