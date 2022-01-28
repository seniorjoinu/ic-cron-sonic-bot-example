use async_trait::async_trait;
use ic_cdk::api::call::CallResult;
use ic_cdk::call;
use ic_cdk::export::candid::{CandidType, Deserialize, Int, Nat, Principal};

#[derive(CandidType, Deserialize)]
pub struct SonicTokenInfo {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub fee: Nat,
    pub totalSupply: Nat,
}

#[derive(CandidType, Deserialize)]
pub struct SonicPairInfo {
    pub id: String,
    pub token0: String,
    pub token1: String,
    pub creator: Principal,
    pub reserve0: Nat,
    pub reserve1: Nat,
    pub price0CumulativeLast: Nat,
    pub price1CumulativeLast: Nat,
    pub kLast: Nat,
    pub blockTimestampLast: Int,
    pub totalSupply: Nat,
    pub lptoken: String,
}

#[derive(CandidType, Deserialize)]
pub struct SonicUserInfo {
    pub balances: Vec<(Principal, Nat)>,
    pub lpBalances: Vec<(String, Nat)>,
}

#[derive(CandidType, Deserialize)]
pub struct SonicSwapInfo {
    pub owner: Principal,
    pub cycles: Nat,
    pub tokens: Vec<SonicTokenInfo>,
    pub pairs: Vec<SonicPairInfo>,
}

#[derive(CandidType, Deserialize)]
pub enum MotokoResult<T, E> {
    ok(T),
    err(E),
}

pub type SonicTxReceipt = MotokoResult<Nat, String>;

impl<T, E> MotokoResult<T, E> {
    pub fn to_res(self) -> Result<T, E> {
        match self {
            MotokoResult::ok(t) => Ok(t),
            MotokoResult::err(e) => Err(e),
        }
    }
}

#[derive(CandidType, Deserialize)]
pub enum SonicDetailValue {
    I64(i64),
    U64(u64),
    Vec(Vec<SonicDetailValue>),
    Slice(Vec<u8>),
    Text(String),
    True,
    False,
    Float(f64),
    Principal(Principal),
}

#[derive(CandidType, Deserialize)]
pub struct SonicTxRecord {
    pub caller: Principal,
    pub operation: String,
    pub details: Vec<(String, SonicDetailValue)>,
    pub time: u64,
}

#[async_trait]
pub trait Sonic {
    // ------------ SWAP API --------------

    async fn swap_exact_tokens_for_tokens(
        &self,
        amount_in: Nat,
        amount_out_min: Nat,
        path: Vec<String>,
        to: Principal,
        deadline: Int,
    ) -> CallResult<(SonicTxReceipt,)>;

    async fn swap_tokens_for_exact_tokens(
        &self,
        amount_out: Nat,
        amount_in_max: Nat,
        path: Vec<String>,
        to: Principal,
        deadline: Int,
    ) -> CallResult<(SonicTxReceipt,)>;

    async fn get_pair(
        &self,
        token0: Principal,
        token1: Principal,
    ) -> CallResult<(Option<SonicPairInfo>,)>;

    async fn get_all_pairs(&self) -> CallResult<(Vec<SonicPairInfo>,)>;

    async fn get_num_pairs(&self) -> CallResult<(Nat,)>;

    // -------------- LIQUIDITY API ----------------

    async fn add_liquidity(
        &self,
        token0: Principal,
        token1: Principal,
        amount0_desired: Nat,
        amount1_desired: Nat,
        amount0_min: Nat,
        amount1_min: Nat,
        deadline: Int,
    ) -> CallResult<(SonicTxReceipt,)>;

    async fn remove_liquidity(
        &self,
        token0: Principal,
        token1: Principal,
        lp_amount: Nat,
        amount0_min: Nat,
        amount1_min: Nat,
        deadline: Int,
    ) -> CallResult<(SonicTxReceipt,)>;

    async fn get_user_LP_balances(&self, user: Principal) -> CallResult<(Vec<(String, Nat)>,)>;

    // ------------------ ASSETS API -------------------

    async fn add_token(&self, token_id: Principal) -> CallResult<(SonicTxReceipt,)>;

    async fn create_pair(
        &self,
        token0: Principal,
        token1: Principal,
    ) -> CallResult<(SonicTxReceipt,)>;

    async fn deposit(&self, token_id: Principal, value: Nat) -> CallResult<(SonicTxReceipt,)>;

    async fn withdraw(&self, token_id: Principal, value: Nat) -> CallResult<(SonicTxReceipt,)>;

    async fn transfer(&self, token_id: String, to: Principal, value: Nat) -> CallResult<(bool,)>;

    async fn approve(
        &self,
        token_id: String,
        spender: Principal,
        value: Nat,
    ) -> CallResult<(bool,)>;

    async fn transfer_from(
        &self,
        token_id: String,
        from: Principal,
        to: Principal,
        value: Nat,
    ) -> CallResult<(SonicTxReceipt,)>;

    async fn get_supported_token_list(&self) -> CallResult<(SonicTokenInfo,)>;

    async fn balance_of(&self, token_id: String, who: Principal) -> CallResult<(Nat,)>;

    async fn allowance(
        &self,
        token_id: String,
        owner: Principal,
        spender: Principal,
    ) -> CallResult<(Nat,)>;

    async fn total_supply(&self, token_id: String) -> CallResult<(Nat,)>;

    async fn name(&self, token_id: String) -> CallResult<(String,)>;

    async fn decimals(&self, token_id: String) -> CallResult<(Nat,)>;

    async fn symbol(&self, token_id: String) -> CallResult<(String,)>;

    // ------------------- OTHER API ----------------------

    async fn get_user_info(&self, user: Principal) -> CallResult<(SonicUserInfo,)>;

    async fn get_swap_info(&self) -> CallResult<(SonicSwapInfo,)>;
}

#[async_trait]
impl Sonic for Principal {
    async fn swap_exact_tokens_for_tokens(
        &self,
        amount_in: Nat,
        amount_out_min: Nat,
        path: Vec<String>,
        to: Principal,
        deadline: Int,
    ) -> CallResult<(SonicTxReceipt,)> {
        call(
            *self,
            "swapExactTokensForTokens",
            (amount_in, amount_out_min, path, to, deadline),
        )
        .await
    }

    async fn swap_tokens_for_exact_tokens(
        &self,
        amount_out: Nat,
        amount_in_max: Nat,
        path: Vec<String>,
        to: Principal,
        deadline: Int,
    ) -> CallResult<(SonicTxReceipt,)> {
        call(
            *self,
            "swapTokensForExactTokens",
            (amount_out, amount_in_max, path, to, deadline),
        )
        .await
    }

    async fn get_pair(
        &self,
        token0: Principal,
        token1: Principal,
    ) -> CallResult<(Option<SonicPairInfo>,)> {
        call(*self, "getPair", (token0, token1)).await
    }

    async fn get_all_pairs(&self) -> CallResult<(Vec<SonicPairInfo>,)> {
        call(*self, "getAllPairs", ()).await
    }

    async fn get_num_pairs(&self) -> CallResult<(Nat,)> {
        call(*self, "getNumPairs", ()).await
    }

    async fn add_liquidity(
        &self,
        token0: Principal,
        token1: Principal,
        amount0_desired: Nat,
        amount1_desired: Nat,
        amount0_min: Nat,
        amount1_min: Nat,
        deadline: Int,
    ) -> CallResult<(SonicTxReceipt,)> {
        call(
            *self,
            "addLiquidity",
            (
                token0,
                token1,
                amount0_desired,
                amount1_desired,
                amount0_min,
                amount1_min,
                deadline,
            ),
        )
        .await
    }

    async fn remove_liquidity(
        &self,
        token0: Principal,
        token1: Principal,
        lp_amount: Nat,
        amount0_min: Nat,
        amount1_min: Nat,
        deadline: Int,
    ) -> CallResult<(SonicTxReceipt,)> {
        call(
            *self,
            "removeLiquidity",
            (
                token0,
                token1,
                lp_amount,
                amount0_min,
                amount1_min,
                deadline,
            ),
        )
        .await
    }

    async fn get_user_LP_balances(&self, user: Principal) -> CallResult<(Vec<(String, Nat)>,)> {
        call(*self, "getUserLPBalances", (user,)).await
    }

    async fn add_token(&self, token_id: Principal) -> CallResult<(SonicTxReceipt,)> {
        call(*self, "addToken", (token_id,)).await
    }

    async fn create_pair(
        &self,
        token0: Principal,
        token1: Principal,
    ) -> CallResult<(SonicTxReceipt,)> {
        call(*self, "createPair", (token0, token1)).await
    }

    async fn deposit(&self, token_id: Principal, value: Nat) -> CallResult<(SonicTxReceipt,)> {
        call(*self, "deposit", (token_id, value)).await
    }

    async fn withdraw(&self, token_id: Principal, value: Nat) -> CallResult<(SonicTxReceipt,)> {
        call(*self, "withdraw", (token_id, value)).await
    }

    async fn transfer(&self, token_id: String, to: Principal, value: Nat) -> CallResult<(bool,)> {
        call(*self, "transfer", (token_id, to, value)).await
    }

    async fn approve(
        &self,
        token_id: String,
        spender: Principal,
        value: Nat,
    ) -> CallResult<(bool,)> {
        call(*self, "approve", (token_id, spender, value)).await
    }

    async fn transfer_from(
        &self,
        token_id: String,
        from: Principal,
        to: Principal,
        value: Nat,
    ) -> CallResult<(SonicTxReceipt,)> {
        call(*self, "transferFrom", (token_id, from, to, value)).await
    }

    async fn get_supported_token_list(&self) -> CallResult<(SonicTokenInfo,)> {
        call(*self, "getSupportedTokenList", ()).await
    }

    async fn balance_of(&self, token_id: String, who: Principal) -> CallResult<(Nat,)> {
        call(*self, "balanceOf", (token_id, who)).await
    }

    async fn allowance(
        &self,
        token_id: String,
        owner: Principal,
        spender: Principal,
    ) -> CallResult<(Nat,)> {
        call(*self, "allowance", (token_id, owner, spender)).await
    }

    async fn total_supply(&self, token_id: String) -> CallResult<(Nat,)> {
        call(*self, "totalSupply", (token_id,)).await
    }

    async fn name(&self, token_id: String) -> CallResult<(String,)> {
        call(*self, "name", (token_id,)).await
    }

    async fn decimals(&self, token_id: String) -> CallResult<(Nat,)> {
        call(*self, "decimals", (token_id,)).await
    }

    async fn symbol(&self, token_id: String) -> CallResult<(String,)> {
        call(*self, "symbol", (token_id,)).await
    }

    async fn get_user_info(&self, user: Principal) -> CallResult<(SonicUserInfo,)> {
        call(*self, "getUserInfo", (user,)).await
    }

    async fn get_swap_info(&self) -> CallResult<(SonicSwapInfo,)> {
        call(*self, "getSwapInfo", ()).await
    }
}
