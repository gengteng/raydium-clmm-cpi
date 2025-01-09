use anchor_lang::prelude::*;

use crate::account::*;

#[account]
pub struct AccountPlaceholder {}

#[derive(Accounts)]
#[instruction(index: u16)]
pub struct CreateAmmConfig<'info> {
    /// Address to be set as protocol owner.
    #[account(mut)]
    pub owner: Signer<'info>,

    /// Initialize config state account to store protocol owner address and fee rates.
    #[account(mut)]
    pub amm_config: Box<Account<'info, AmmConfig>>,

    pub system_program: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct UpdateAmmConfig<'info> {
    pub owner: Signer<'info>,

    /// Amm config account to be changed
    #[account(mut)]
    pub amm_config: Box<Account<'info, AmmConfig>>,
}

#[derive(Accounts)]
pub struct CreatePool<'info> {
    /// Address paying to create the pool. Can be anyone
    #[account(mut)]
    pub pool_creator: Signer<'info>,

    /// Which config the pool belongs to.
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// Initialize an account to store the pool state
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// Token_0 mint, the key must be smaller then token_1 mint.
    pub token_mint_0: Account<'info, AccountPlaceholder>,

    /// Token_1 mint
    pub token_mint_1: Account<'info, AccountPlaceholder>,

    /// Token_0 vault for the pool
    #[account(mut)]
    pub token_vault_0: Account<'info, AccountPlaceholder>,

    /// Token_1 vault for the pool
    #[account(mut)]
    pub token_vault_1: Account<'info, AccountPlaceholder>,

    /// Initialize an account to store oracle observations
    #[account(mut)]
    pub observation_state: AccountLoader<'info, ObservationState>,

    /// Initialize an account to store if a tick array is initialized.
    #[account(mut)]
    pub tick_array_bitmap: Account<'info, AccountPlaceholder>,

    /// Spl token program or token program 2022
    pub token_program_0: Account<'info, AccountPlaceholder>,
    /// Spl token program or token program 2022
    pub token_program_1: Account<'info, AccountPlaceholder>,
    /// To create a new program account
    pub system_program: Account<'info, AccountPlaceholder>,
    /// Sysvar for program account
    pub rent: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct UpdatePoolStatus<'info> {
    pub authority: Signer<'info>,

    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,
}

#[derive(Accounts)]
pub struct CreateOperationAccount<'info> {
    /// Address to be set as operation account owner.
    #[account(mut)]
    pub owner: Signer<'info>,

    /// Initialize operation state account to store operation owner address and white list mint.
    #[account(mut)]
    pub operation_state: AccountLoader<'info, OperationState>,

    pub system_program: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct UpdateOperationAccount<'info> {
    /// Address to be set as operation account owner.
    pub owner: Signer<'info>,

    /// Initialize operation state account to store operation owner address and white list mint.
    #[account(mut)]
    pub operation_state: AccountLoader<'info, OperationState>,

    pub system_program: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct TransferRewardOwner<'info> {
    /// Address to be set as operation account owner.
    pub authority: Signer<'info>,

    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,
}

#[derive(Accounts)]
pub struct InitializeReward<'info> {
    /// The founder deposit reward token to vault
    #[account(mut)]
    pub reward_funder: Signer<'info>,

    // The funder's reward token account
    #[account(mut)]
    pub funder_token_account: Account<'info, AccountPlaceholder>,

    /// For check the reward_funder authority
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// Set reward for this pool
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// load info from the account to judge reward permission
    pub operation_state: AccountLoader<'info, OperationState>,

    /// Reward mint
    pub reward_token_mint: Account<'info, AccountPlaceholder>,

    /// A pda, reward vault
    #[account(mut)]
    pub reward_token_vault: Account<'info, AccountPlaceholder>,

    pub reward_token_program: Account<'info, AccountPlaceholder>,
    pub system_program: Account<'info, AccountPlaceholder>,
    pub rent: Account<'info, AccountPlaceholder>,
}

#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize, Debug, PartialEq)]
pub struct InitializeRewardParam {
    /// Reward open time
    pub open_time: u64,
    /// Reward end time
    pub end_time: u64,
    /// Token reward per second are earned per unit of liquidity
    pub emissions_per_second_x64: u128,
}

#[derive(Accounts)]
pub struct CollectRemainingRewards<'info> {
    /// The founder who init reward info in berfore
    pub reward_funder: Signer<'info>,
    /// The funder's reward token account
    #[account(mut)]
    pub funder_token_account: Account<'info, AccountPlaceholder>,
    /// Set reward for this pool
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,
    /// Reward vault transfer remaining token to founder token account
    pub reward_token_vault: Account<'info, AccountPlaceholder>,
    /// The mint of reward token vault
    pub reward_vault_mint: Account<'info, AccountPlaceholder>,
    pub token_program: Account<'info, AccountPlaceholder>,
    /// Token program 2022
    pub token_program_2022: Account<'info, AccountPlaceholder>,

    /// memo program
    pub memo_program: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct UpdateRewardInfos<'info> {
    /// The liquidity pool for which reward info to update
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,
}

#[derive(Accounts)]
pub struct SetRewardParams<'info> {
    /// Address to be set as protocol owner. It pays to create factory state account.
    pub authority: Signer<'info>,

    pub amm_config: Box<Account<'info, AmmConfig>>,

    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// load info from the account to judge reward permission
    pub operation_state: AccountLoader<'info, OperationState>,

    /// Token program
    pub token_program: Account<'info, AccountPlaceholder>,
    /// Token program 2022
    pub token_program_2022: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct CollectProtocolFee<'info> {
    /// Only admin or config owner can collect fee now
    pub owner: Signer<'info>,

    /// Pool state stores accumulated protocol fee amount
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// Amm config account stores owner
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// The address that holds pool tokens for token_0
    #[account(mut)]
    pub token_vault_0: Account<'info, AccountPlaceholder>,

    /// The address that holds pool tokens for token_1
    #[account(mut)]
    pub token_vault_1: Account<'info, AccountPlaceholder>,

    /// The mint of token vault 0
    pub vault_0_mint: Account<'info, AccountPlaceholder>,

    /// The mint of token vault 1
    pub vault_1_mint: Account<'info, AccountPlaceholder>,

    /// The address that receives the collected token_0 protocol fees
    #[account(mut)]
    pub recipient_token_account_0: Account<'info, AccountPlaceholder>,

    /// The address that receives the collected token_1 protocol fees
    #[account(mut)]
    pub recipient_token_account_1: Account<'info, AccountPlaceholder>,

    /// The SPL program to perform token transfers
    pub token_program: Account<'info, AccountPlaceholder>,

    /// The SPL program 2022 to perform token transfers
    pub token_program_2022: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct CollectFundFee<'info> {
    /// Only admin or fund_owner can collect fee now
    pub owner: Signer<'info>,

    /// Pool state stores accumulated protocol fee amount
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// Amm config account stores fund_owner
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// The address that holds pool tokens for token_0
    #[account(mut)]
    pub token_vault_0: Account<'info, AccountPlaceholder>,

    /// The address that holds pool tokens for token_1
    #[account(mut)]
    pub token_vault_1: Account<'info, AccountPlaceholder>,

    /// The mint of token vault 0
    pub vault_0_mint: Account<'info, AccountPlaceholder>,

    /// The mint of token vault 1
    pub vault_1_mint: Account<'info, AccountPlaceholder>,

    /// The address that receives the collected token_0 protocol fees
    #[account(mut)]
    pub recipient_token_account_0: Account<'info, AccountPlaceholder>,

    /// The address that receives the collected token_1 protocol fees
    #[account(mut)]
    pub recipient_token_account_1: Account<'info, AccountPlaceholder>,

    /// The SPL program to perform token transfers
    pub token_program: Account<'info, AccountPlaceholder>,

    /// The SPL program 2022 to perform token transfers
    pub token_program_2022: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
#[instruction(tick_lower_index: i32, tick_upper_index: i32,tick_array_lower_start_index:i32,tick_array_upper_start_index:i32)]
pub struct OpenPosition<'info> {
    /// Pays to mint the position
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Receives the position NFT
    pub position_nft_owner: Account<'info, AccountPlaceholder>,

    /// Unique token mint address
    #[account(mut)]
    pub position_nft_mint: Account<'info, AccountPlaceholder>,

    /// Token account where position NFT will be minted
    /// This account created in the contract by cpi to avoid large stack variables
    #[account(mut)]
    pub position_nft_account: Account<'info, AccountPlaceholder>,

    /// To store metaplex metadata
    /// CHECK: Safety check performed inside function body
    #[account(mut)]
    pub metadata_account: Account<'info, AccountPlaceholder>,

    /// Add liquidity for this pool
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// Store the information of market marking in range
    #[account(mut)]
    pub protocol_position: Account<'info, AccountPlaceholder>,

    /// CHECK: Account to store data for the position's lower tick
    #[account(mut)]
    pub tick_array_lower: Account<'info, AccountPlaceholder>,

    /// CHECK: Account to store data for the position's upper tick
    #[account(mut)]
    pub tick_array_upper: Account<'info, AccountPlaceholder>,

    /// personal position state
    #[account(mut)]
    pub personal_position: Account<'info, AccountPlaceholder>,

    /// The token_0 account deposit token to the pool
    #[account(mut)]
    pub token_account_0: Account<'info, AccountPlaceholder>,

    /// The token_1 account deposit token to the pool
    #[account(mut)]
    pub token_account_1: Account<'info, AccountPlaceholder>,

    /// The address that holds pool tokens for token_0
    #[account(mut)]
    pub token_vault_0: Account<'info, AccountPlaceholder>,

    /// The address that holds pool tokens for token_1
    #[account(mut)]
    pub token_vault_1: Account<'info, AccountPlaceholder>,

    /// Sysvar for token mint and ATA creation
    pub rent: Account<'info, AccountPlaceholder>,

    /// Program to create the position manager state account
    pub system_program: Account<'info, AccountPlaceholder>,

    /// Program to create mint account and mint tokens
    pub token_program: Account<'info, AccountPlaceholder>,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: Account<'info, AccountPlaceholder>,

    /// Program to create NFT metadata
    /// CHECK: Metadata program address constraint applied
    pub metadata_program: Account<'info, AccountPlaceholder>,
    // remaining account
    // #[account(
    //     seeds = [
    //         POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(),
    //         pool_state.key().as_ref(),
    //     ],
    //     bump
    // )]
    // pub tick_array_bitmap: AccountLoader<'info, TickArrayBitmapExtension>,
}

#[derive(Accounts)]
#[instruction(tick_lower_index: i32, tick_upper_index: i32,tick_array_lower_start_index:i32,tick_array_upper_start_index:i32)]
pub struct OpenPositionV2<'info> {
    /// Pays to mint the position
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Receives the position NFT
    pub position_nft_owner: Account<'info, AccountPlaceholder>,

    /// Unique token mint address
    #[account(mut)]
    pub position_nft_mint: Account<'info, AccountPlaceholder>,

    /// Token account where position NFT will be minted
    #[account(mut)]
    pub position_nft_account: Account<'info, AccountPlaceholder>,

    /// To store metaplex metadata
    /// CHECK: Safety check performed inside function body
    #[account(mut)]
    pub metadata_account: Account<'info, AccountPlaceholder>,

    /// Add liquidity for this pool
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// Store the information of market marking in range
    #[account(mut)]
    pub protocol_position: Account<'info, AccountPlaceholder>,

    /// CHECK: Account to store data for the position's lower tick
    #[account(mut)]
    pub tick_array_lower: Account<'info, AccountPlaceholder>,

    /// CHECK: Account to store data for the position's upper tick
    #[account(mut)]
    pub tick_array_upper: Account<'info, AccountPlaceholder>,

    /// personal position state
    #[account(mut)]
    pub personal_position: Account<'info, AccountPlaceholder>,

    /// The token_0 account deposit token to the pool
    #[account(mut)]
    pub token_account_0: Account<'info, AccountPlaceholder>,

    /// The token_1 account deposit token to the pool
    #[account(mut)]
    pub token_account_1: Account<'info, AccountPlaceholder>,

    /// The address that holds pool tokens for token_0
    #[account(mut)]
    pub token_vault_0: Account<'info, AccountPlaceholder>,

    /// The address that holds pool tokens for token_1
    #[account(mut)]
    pub token_vault_1: Account<'info, AccountPlaceholder>,

    /// Sysvar for token mint and ATA creation
    pub rent: Account<'info, AccountPlaceholder>,

    /// Program to create the position manager state account
    pub system_program: Account<'info, AccountPlaceholder>,

    /// Program to create mint account and mint tokens
    pub token_program: Account<'info, AccountPlaceholder>,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: Account<'info, AccountPlaceholder>,

    /// Program to create NFT metadata
    /// CHECK: Metadata program address constraint applied
    pub metadata_program: Account<'info, AccountPlaceholder>,
    /// Program to create mint account and mint tokens
    pub token_program_2022: Account<'info, AccountPlaceholder>,
    /// The mint of token vault 0
    #[account(mut)]
    pub vault_0_mint: Account<'info, AccountPlaceholder>,
    /// The mint of token vault 1
    #[account(mut)]
    pub vault_1_mint: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
#[instruction(tick_lower_index: i32, tick_upper_index: i32,tick_array_lower_start_index:i32,tick_array_upper_start_index:i32)]
pub struct OpenPositionWithToken22Nft<'info> {
    /// Pays to mint the position
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Receives the position NFT
    pub position_nft_owner: Account<'info, AccountPlaceholder>,

    /// Unique token mint address, initialize in constract
    #[account(mut)]
    pub position_nft_mint: Signer<'info>,

    /// CHECK: ATA address where position NFT will be minted, initialize in constract
    #[account(mut)]
    pub position_nft_account: Account<'info, AccountPlaceholder>,

    /// Add liquidity for this pool
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// Store the information of market marking in range
    #[account(mut)]
    pub protocol_position: Account<'info, AccountPlaceholder>,

    /// CHECK: Account to store data for the position's lower tick
    #[account(mut)]
    pub tick_array_lower: Account<'info, AccountPlaceholder>,

    /// CHECK: Account to store data for the position's upper tick
    #[account(mut)]
    pub tick_array_upper: Account<'info, AccountPlaceholder>,

    /// personal position state
    #[account(mut)]
    pub personal_position: Account<'info, AccountPlaceholder>,

    /// The token_0 account deposit token to the pool
    #[account(mut)]
    pub token_account_0: Account<'info, AccountPlaceholder>,

    /// The token_1 account deposit token to the pool
    #[account(mut)]
    pub token_account_1: Account<'info, AccountPlaceholder>,

    /// The address that holds pool tokens for token_0
    #[account(mut)]
    pub token_vault_0: Account<'info, AccountPlaceholder>,

    /// The address that holds pool tokens for token_1
    #[account(mut)]
    pub token_vault_1: Account<'info, AccountPlaceholder>,

    /// Sysvar for token mint and ATA creation
    pub rent: Account<'info, AccountPlaceholder>,

    /// Program to create the position manager state account
    pub system_program: Account<'info, AccountPlaceholder>,

    /// Program to transfer for token account
    pub token_program: Account<'info, AccountPlaceholder>,

    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: Account<'info, AccountPlaceholder>,

    /// Program to create NFT mint/token account and transfer for token22 account
    pub token_program_2022: Account<'info, AccountPlaceholder>,

    /// The mint of token vault 0
    #[account(mut)]
    pub vault_0_mint: Account<'info, AccountPlaceholder>,

    /// The mint of token vault 1
    #[account(mut)]
    pub vault_1_mint: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    /// The position nft owner
    #[account(mut)]
    pub nft_owner: Signer<'info>,

    /// Mint address bound to the personal position.
    #[account(mut)]
    pub position_nft_mint: Account<'info, AccountPlaceholder>,

    /// User token account where position NFT be minted to
    #[account(mut)]
    pub position_nft_account: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub personal_position: Account<'info, AccountPlaceholder>,

    /// System program to close the position state account
    pub system_program: Account<'info, AccountPlaceholder>,

    /// Token/Token2022 program to close token/mint account
    pub token_program: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct IncreaseLiquidity<'info> {
    /// Pays to mint the position
    pub nft_owner: Signer<'info>,

    /// The token account for nft
    pub nft_account: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    #[account(mut)]
    pub protocol_position: Account<'info, AccountPlaceholder>,

    /// Increase liquidity for this position
    #[account(mut)]
    pub personal_position: Account<'info, AccountPlaceholder>,

    /// Stores init state for the lower tick
    #[account(mut)]
    pub tick_array_lower: Account<'info, AccountPlaceholder>,

    /// Stores init state for the upper tick
    #[account(mut)]
    pub tick_array_upper: Account<'info, AccountPlaceholder>,

    /// The payer's token account for token_0
    #[account(mut)]
    pub token_account_0: Account<'info, AccountPlaceholder>,

    /// The token account spending token_1 to mint the position
    #[account(mut)]
    pub token_account_1: Account<'info, AccountPlaceholder>,

    /// The address that holds pool tokens for token_0
    #[account(mut)]
    pub token_vault_0: Account<'info, AccountPlaceholder>,

    /// The address that holds pool tokens for token_1
    #[account(mut)]
    pub token_vault_1: Account<'info, AccountPlaceholder>,

    /// Program to create mint account and mint tokens
    pub token_program: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct IncreaseLiquidityV2<'info> {
    /// Pays to mint the position
    pub nft_owner: Signer<'info>,

    /// The token account for nft
    pub nft_account: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    #[account(mut)]
    pub protocol_position: Account<'info, AccountPlaceholder>,

    /// Increase liquidity for this position
    #[account(mut)]
    pub personal_position: Account<'info, AccountPlaceholder>,

    /// Stores init state for the lower tick
    #[account(mut)]
    pub tick_array_lower: Account<'info, AccountPlaceholder>,

    /// Stores init state for the upper tick
    #[account(mut)]
    pub tick_array_upper: Account<'info, AccountPlaceholder>,

    /// The payer's token account for token_0
    #[account(mut)]
    pub token_account_0: Account<'info, AccountPlaceholder>,

    /// The token account spending token_1 to mint the position
    #[account(mut)]
    pub token_account_1: Account<'info, AccountPlaceholder>,

    /// The address that holds pool tokens for token_0
    #[account(mut)]
    pub token_vault_0: Account<'info, AccountPlaceholder>,

    /// The address that holds pool tokens for token_1
    #[account(mut)]
    pub token_vault_1: Account<'info, AccountPlaceholder>,

    /// Program to create mint account and mint tokens
    pub token_program: Account<'info, AccountPlaceholder>,

    /// Token program 2022
    pub token_program_2022: Account<'info, AccountPlaceholder>,

    /// The mint of token vault 0
    #[account(mut)]
    pub vault_0_mint: Account<'info, AccountPlaceholder>,

    /// The mint of token vault 1
    #[account(mut)]
    pub vault_1_mint: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct DecreaseLiquidity<'info> {
    /// The position owner or delegated authority
    pub nft_owner: Signer<'info>,

    /// The token account for the tokenized position
    pub nft_account: Account<'info, AccountPlaceholder>,

    /// Decrease liquidity for this position
    #[account(mut)]
    pub personal_position: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    #[account(mut)]
    pub protocol_position: Account<'info, AccountPlaceholder>,

    /// Token_0 vault
    #[account(mut)]
    pub token_vault_0: Account<'info, AccountPlaceholder>,

    /// Token_1 vault
    #[account(mut)]
    pub token_vault_1: Account<'info, AccountPlaceholder>,

    /// Stores init state for the lower tick
    #[account(mut)]
    pub tick_array_lower: Account<'info, AccountPlaceholder>,

    /// Stores init state for the upper tick
    #[account(mut)]
    pub tick_array_upper: Account<'info, AccountPlaceholder>,

    /// The destination token account for receive amount_0
    #[account(mut)]
    pub recipient_token_account_0: Account<'info, AccountPlaceholder>,

    /// The destination token account for receive amount_1
    #[account(mut)]
    pub recipient_token_account_1: Account<'info, AccountPlaceholder>,

    /// SPL program to transfer out tokens
    pub token_program: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct DecreaseLiquidityV2<'info> {
    /// The position owner or delegated authority
    pub nft_owner: Signer<'info>,

    /// The token account for the tokenized position
    pub nft_account: Account<'info, AccountPlaceholder>,

    /// Decrease liquidity for this position
    #[account(mut)]
    pub personal_position: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    #[account(mut)]
    pub protocol_position: Account<'info, AccountPlaceholder>,

    /// Token_0 vault
    #[account(mut)]
    pub token_vault_0: Account<'info, AccountPlaceholder>,

    /// Token_1 vault
    #[account(mut)]
    pub token_vault_1: Account<'info, AccountPlaceholder>,

    /// Stores init state for the lower tick
    #[account(mut)]
    pub tick_array_lower: Account<'info, AccountPlaceholder>,

    /// Stores init state for the upper tick
    #[account(mut)]
    pub tick_array_upper: Account<'info, AccountPlaceholder>,

    /// The destination token account for receive amount_0
    #[account(mut)]
    pub recipient_token_account_0: Account<'info, AccountPlaceholder>,

    /// The destination token account for receive amount_1
    #[account(mut)]
    pub recipient_token_account_1: Account<'info, AccountPlaceholder>,

    /// SPL program to transfer out tokens
    pub token_program: Account<'info, AccountPlaceholder>,

    /// Token program 2022
    pub token_program_2022: Account<'info, AccountPlaceholder>,

    /// memo program
    /// CHECK:
    pub memo_program: Account<'info, AccountPlaceholder>,

    /// The mint of token vault 0
    #[account(mut)]
    pub vault_0_mint: Account<'info, AccountPlaceholder>,

    /// The mint of token vault 1
    #[account(mut)]
    pub vault_1_mint: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct SwapSingle<'info> {
    /// The user performing the swap
    pub payer: Signer<'info>,

    /// The factory state to read protocol fees
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// The program account of the pool in which the swap will be performed
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// The user token account for input token
    #[account(mut)]
    pub input_token_account: Account<'info, AccountPlaceholder>,

    /// The user token account for output token
    #[account(mut)]
    pub output_token_account: Account<'info, AccountPlaceholder>,

    /// The vault token account for input token
    #[account(mut)]
    pub input_vault: Account<'info, AccountPlaceholder>,

    /// The vault token account for output token
    #[account(mut)]
    pub output_vault: Account<'info, AccountPlaceholder>,

    /// The program account for the most recent oracle observation
    #[account(mut)]
    pub observation_state: AccountLoader<'info, ObservationState>,

    /// SPL program for token transfers
    pub token_program: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub tick_array: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct SwapSingleV2<'info> {
    /// The user performing the swap
    pub payer: Signer<'info>,

    /// The factory state to read protocol fees
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// The program account of the pool in which the swap will be performed
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// The user token account for input token
    #[account(mut)]
    pub input_token_account: Account<'info, AccountPlaceholder>,

    /// The user token account for output token
    #[account(mut)]
    pub output_token_account: Account<'info, AccountPlaceholder>,

    /// The vault token account for input token
    #[account(mut)]
    pub input_vault: Account<'info, AccountPlaceholder>,

    /// The vault token account for output token
    #[account(mut)]
    pub output_vault: Account<'info, AccountPlaceholder>,

    /// The program account for the most recent oracle observation
    #[account(mut)]
    pub observation_state: AccountLoader<'info, ObservationState>,

    /// SPL program for token transfers
    pub token_program: Account<'info, AccountPlaceholder>,

    /// SPL program 2022 for token transfers
    pub token_program_2022: Account<'info, AccountPlaceholder>,

    /// CHECK:
    pub memo_program: Account<'info, AccountPlaceholder>,

    /// The mint of token vault 0
    #[account(mut)]
    pub input_vault_mint: Account<'info, AccountPlaceholder>,

    /// The mint of token vault 1
    #[account(mut)]
    pub output_vault_mint: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct SwapRouterBaseIn<'info> {
    /// The user performing the swap
    pub payer: Signer<'info>,

    /// The token account that pays input tokens for the swap
    #[account(mut)]
    pub input_token_account: Account<'info, AccountPlaceholder>,

    /// The mint of input token
    #[account(mut)]
    pub input_token_mint: Account<'info, AccountPlaceholder>,

    /// SPL program for token transfers
    pub token_program: Account<'info, AccountPlaceholder>,

    /// SPL program 2022 for token transfers
    pub token_program_2022: Account<'info, AccountPlaceholder>,

    /// CHECK:
    pub memo_program: Account<'info, AccountPlaceholder>,
}
