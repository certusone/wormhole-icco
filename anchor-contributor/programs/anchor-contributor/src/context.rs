use anchor_lang::{
    prelude::*,
    solana_program::sysvar::{clock, rent},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, ID},
};
use std::str::FromStr;

use crate::{
    constants::*,
    state::{Buyer, Custodian, Sale},
};

/// Context allows contract owner to create an account that acts
/// to hold all associated token accounts for all sales.
/// See `create_custodian` instruction in lib.rs.
///
/// Mutable
/// * `custodian`
/// * `owner` (signer)
#[derive(Accounts)]
pub struct CreateCustodian<'info> {
    #[account(
        init,
        payer = owner,
        seeds = [
            SEED_PREFIX_CUSTODIAN.as_bytes(),
        ],
        bump,
        space = 8 + Custodian::MAXIMUM_SIZE,
    )]
    pub custodian: Account<'info, Custodian>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Context provides all accounts required for someone to initialize a sale
/// with a signed VAA sent by the conductor. A `Sale` is created at this step,
/// which will be used for future actions.
/// See `init_sale` instruction in lib.rs.
///
/// /// Immutable
/// * `custodian`
/// * `core_bridge_vaa`
/// * `sale_token_mint`
/// * `custodian_sale_token_acct`
///
/// Mutable
/// * `sale`
/// * `owner` (signer)
#[derive(Accounts)]
pub struct InitSale<'info> {
    #[account(
        seeds = [
            SEED_PREFIX_CUSTODIAN.as_bytes(),
        ],
        bump,
    )]
    pub custodian: Account<'info, Custodian>,

    #[account(
        init,
        seeds = [
            SEED_PREFIX_SALE.as_bytes(),
            &Custodian::get_sale_id_from_vaa(&core_bridge_vaa)?,
        ],
        payer = owner,
        bump,
        space = 8 + Sale::MAXIMUM_SIZE
    )]
    pub sale: Account<'info, Sale>,

    #[account(
        constraint = core_bridge_vaa.owner.key() == Pubkey::from_str(CORE_BRIDGE_ADDRESS).unwrap()
    )]
    /// CHECK: This account is owned by Core Bridge so we trust it
    pub core_bridge_vaa: AccountInfo<'info>,
    pub sale_token_mint: Account<'info, Mint>,

    #[
        account(
            constraint = custodian_sale_token_acct.mint == sale_token_mint.key(),
            constraint = custodian_sale_token_acct.owner == custodian.key(),
        )
    ]
    pub custodian_sale_token_acct: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Context provides all accounts required for user to send contribution
/// to ongoing sale.
/// See `contribute` instruction in lib.rs.
///
/// Immutable
/// * `custodian`
///
/// Mutable
/// * `sale`
/// * `buyer`
/// * `buyer_token_acct`
/// * `custodian_token_acct`
/// * `owner` (signer)
#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(
        seeds = [
            SEED_PREFIX_CUSTODIAN.as_bytes(),
        ],
        bump,
    )]
    pub custodian: Account<'info, Custodian>,

    #[account(
        mut,
        seeds = [
            SEED_PREFIX_SALE.as_bytes(),
            &sale.id,
        ],
        bump,
    )]
    pub sale: Account<'info, Sale>,

    #[account(
        init_if_needed,
        seeds = [
            SEED_PREFIX_BUYER.as_bytes(),
            &sale.id,
            &owner.key().as_ref(),
        ],
        payer = owner,
        bump,
        space = 8 + Buyer::MAXIMUM_SIZE,
    )]
    pub buyer: Account<'info, Buyer>,

    #[account(
        mut,
        constraint = buyer_token_acct.mint == custodian_token_acct.mint,
        constraint = buyer_token_acct.owner == owner.key(),
    )]
    pub buyer_token_acct: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = custodian_token_acct.owner == custodian.key(),
    )]
    pub custodian_token_acct: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

/// Context provides all accounts required to attest contributions.
/// See `attest_contributions` instruction in lib.rs.
///
/// Immutable
/// * `sale`
/// * `core_bridge`
/// * `clock`
/// * `rent`
///
/// Mutable
/// * `wormhole_config`
/// * `wormhole_fee_collector`
/// * `wormhole_derived_emitter`
/// * `wormhole_sequence`
/// * `wormhole_message_key`
/// * `owner` (signer)
#[derive(Accounts)]
pub struct AttestContributions<'info> {
    #[account(
        seeds = [
            SEED_PREFIX_SALE.as_bytes(),
            &sale.id,
        ],
        bump,
    )]
    pub sale: Account<'info, Sale>,

    #[account(
        constraint = core_bridge.key() == Pubkey::from_str(CORE_BRIDGE_ADDRESS).unwrap()
    )]
    /// CHECK: If someone passes in the wrong account, Guardians won't read the message
    pub core_bridge: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            b"Bridge".as_ref()
        ],
        bump,
        seeds::program = Pubkey::from_str(CORE_BRIDGE_ADDRESS).unwrap()
    )]
    /// CHECK: If someone passes in the wrong account, Guardians won't read the message
    pub wormhole_config: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            b"fee_collector".as_ref()
        ],
        bump,
        seeds::program = Pubkey::from_str(CORE_BRIDGE_ADDRESS).unwrap()
    )]
    /// CHECK: If someone passes in the wrong account, Guardians won't read the message
    pub wormhole_fee_collector: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            b"emitter".as_ref(),
        ],
        bump
    )]
    /// CHECK: If someone passes in the wrong account, Guardians won't read the message
    pub wormhole_derived_emitter: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            b"Sequence".as_ref(),
            wormhole_derived_emitter.key().to_bytes().as_ref()
        ],
        bump,
        seeds::program = Pubkey::from_str(CORE_BRIDGE_ADDRESS).unwrap()
    )]
    /// CHECK: If someone passes in the wrong account, Guardians won't read the message
    pub wormhole_sequence: AccountInfo<'info>,

    #[account(mut)]
    pub wormhole_message_key: Signer<'info>,

    #[account(
        constraint = clock.key() == clock::id()
    )]
    /// CHECK: The account constraint will make sure it's the right clock var
    pub clock: AccountInfo<'info>,

    #[account(
        constraint = rent.key() == rent::id()
    )]
    /// CHECK: The account constraint will make sure it's the right rent var
    pub rent: AccountInfo<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BridgeSealedContribution<'info> {
    #[account(
        seeds = [
            SEED_PREFIX_CUSTODIAN.as_bytes(),
        ],
        bump,
    )]
    pub custodian: Account<'info, Custodian>,

    #[account(
        mut,
        seeds = [
            SEED_PREFIX_SALE.as_bytes(),
            &sale.id,
        ],
        bump,
    )]
    pub sale: Account<'info, Sale>,

    #[account(
        constraint = custody_ata.owner == &AssociatedToken::id()
    )]
    /// CHECK: Check if owned by ATA Program
    pub custody_ata: AccountInfo<'info>,

    #[account(
        constraint = mint_token_account.owner == &ID
    )]
    /// CHECK: Check if owned by SPL Account
    pub mint_token_account: AccountInfo<'info>,

    /// CHECK: Nullable account
    pub wrapped_meta_key: AccountInfo<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        constraint = token_bridge.key() == Pubkey::from_str(TOKEN_BRIDGE_ADDRESS).unwrap()
    )]
    /// CHECK: Checked in account constraints
    pub token_bridge: AccountInfo<'info>,
    #[account(
        seeds=[b"mint_signer"],
        bump,
        seeds::program = token_bridge.key()
    )]
    /// CHECK: We know what we're doing Mr. Anchor ;)
    pub token_mint_signer: AccountInfo<'info>,

    #[account(
        seeds=[b"authority_signer"],
        bump,
        seeds::program = token_bridge.key()
    )]
    /// CHECK: Token Bridge Authority Signer
    pub token_bridge_authority_signer: AccountInfo<'info>,

    #[account(
        seeds = [
            b"config".as_ref()
        ],
        bump,
        seeds::program = Pubkey::from_str(TOKEN_BRIDGE_ADDRESS).unwrap(),
        mut
    )]
    /// CHECK: If someone passes in the wrong account, Guardians won't read the message
    pub token_config: AccountInfo<'info>,

    #[account(
        constraint = core_bridge.key() == Pubkey::from_str(CORE_BRIDGE_ADDRESS).unwrap()
    )]
    /// CHECK: If someone passes in the wrong account, Guardians won't read the message
    pub core_bridge: AccountInfo<'info>,
    #[account(
        seeds = [
            b"Bridge".as_ref()
        ],
        bump,
        seeds::program = Pubkey::from_str(CORE_BRIDGE_ADDRESS).unwrap(),
        mut
    )]
    /// CHECK: If someone passes in the wrong account, Guardians won't read the message
    pub wormhole_config: AccountInfo<'info>,
    #[account(
        seeds = [
            b"fee_collector".as_ref()
        ],
        bump,
        seeds::program = Pubkey::from_str(CORE_BRIDGE_ADDRESS).unwrap(),
        mut
    )]
    /// CHECK: If someone passes in the wrong account, Guardians won't read the message
    pub wormhole_fee_collector: AccountInfo<'info>,
    #[account(
        seeds = [
            b"emitter".as_ref(),
        ],
        bump,
        mut
    )]
    /// CHECK: If someone passes in the wrong account, Guardians won't read the message
    pub wormhole_derived_emitter: AccountInfo<'info>,
    #[account(
        seeds = [
            b"Sequence".as_ref(),
            wormhole_derived_emitter.key().to_bytes().as_ref()
        ],
        bump,
        seeds::program = Pubkey::from_str(CORE_BRIDGE_ADDRESS).unwrap(),
        mut
    )]
    /// CHECK: If someone passes in the wrong account, Guardians won't read the message
    pub wormhole_sequence: AccountInfo<'info>,
    #[account(mut)]
    pub wormhole_message_key: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        constraint = clock.key() == clock::id()
    )]
    /// CHECK: The account constraint will make sure it's the right clock var
    pub clock: AccountInfo<'info>,
    #[account(
        constraint = rent.key() == rent::id()
    )]
    /// CHECK: The account constraint will make sure it's the right rent var
    pub rent: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

/// Context provides all accounts required for someone to abort a sale
/// with a signed VAA sent by the conductor (sale didn't meet min raise).
/// See `abort_sale` instruction in lib.rs.
///
/// Immutable
/// * `custodian`
/// * `core_bridge_vaa`
///
/// Mutable
/// * `sale`
/// * `owner` (signer)
#[derive(Accounts)]
pub struct AbortSale<'info> {
    #[account(
        seeds = [
            SEED_PREFIX_CUSTODIAN.as_bytes(),
        ],
        bump,
    )]
    pub custodian: Account<'info, Custodian>,

    #[account(
        mut,
        seeds = [
            SEED_PREFIX_SALE.as_bytes(),
            &sale.id,
        ],
        bump,
    )]
    pub sale: Account<'info, Sale>,

    #[account(
        constraint = core_bridge_vaa.owner.key() == Pubkey::from_str(CORE_BRIDGE_ADDRESS).unwrap()
    )]
    /// CHECK: This account is owned by Core Bridge so we trust it
    pub core_bridge_vaa: AccountInfo<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Context provides all accounts required for someone to seal a sale
/// with a signed VAA sent by the conductor (sale met at least min raise).
/// See `seal_sale` instruction in lib.rs.
///
/// Immutable
/// * `custodian`
/// * `core_bridge_vaa`
/// * `custodian_sale_token_acct`
///
/// Mutable
/// * `sale`
/// * `owner` (signer)
#[derive(Accounts)]
pub struct SealSale<'info> {
    #[account(
        seeds = [
            SEED_PREFIX_CUSTODIAN.as_bytes(),
        ],
        bump,
    )]
    pub custodian: Account<'info, Custodian>,

    #[account(
        mut,
        seeds = [
            SEED_PREFIX_SALE.as_bytes(),
            &sale.id,
        ],
        bump,
    )]
    pub sale: Account<'info, Sale>,

    #[account(
        constraint = core_bridge_vaa.owner.key() == Pubkey::from_str(CORE_BRIDGE_ADDRESS).unwrap()
    )]
    /// CHECK: This account is owned by Core Bridge so we trust it
    pub core_bridge_vaa: AccountInfo<'info>,

    #[account(
        constraint = custodian_sale_token_acct.mint == sale.sale_token_mint,
        constraint = custodian_sale_token_acct.owner == custodian.key(),
    )]
    pub custodian_sale_token_acct: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Context provides all accounts required for user to claim his allocation
/// and excess contributions after the sale has been sealed.
/// See `claim_allocation` instruction in lib.rs.
///
/// Immutable
/// * `custodian`
/// * `sale`
///
/// Mutable
/// * `buyer`
/// * `custodian_sale_token_acct`
/// * `buyer_sale_token_acct`
/// * `owner` (signer)
///
/// NOTE: With `claim_allocation`, remaining accounts are passed in
/// depending on however many accepted tokens there are for a given sale.
#[derive(Accounts)]
pub struct ClaimAllocation<'info> {
    #[account(
        seeds = [
            SEED_PREFIX_CUSTODIAN.as_bytes(),
        ],
        bump,
    )]
    pub custodian: Account<'info, Custodian>,

    #[account(
        seeds = [
            SEED_PREFIX_SALE.as_bytes(),
            &sale.id,
        ],
        bump,
    )]
    pub sale: Account<'info, Sale>,

    #[account(
        mut,
        seeds = [
            SEED_PREFIX_BUYER.as_bytes(),
            &sale.id,
            &owner.key().as_ref(),
        ],
        bump,
    )]
    pub buyer: Account<'info, Buyer>,

    #[account(
        mut,
        constraint = custodian_sale_token_acct.mint == sale.sale_token_mint,
        constraint = custodian_sale_token_acct.owner == custodian.key(),
    )]
    pub custodian_sale_token_acct: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = buyer_sale_token_acct.mint == sale.sale_token_mint,
        constraint = buyer_sale_token_acct.owner == owner.key(),
    )]
    pub buyer_sale_token_acct: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

/// Context provides all accounts required for user to claim his refunds
/// after the sale has been aborted.
/// See `claim_refunds` instruction in lib.rs.
///
/// /// Immutable
/// * `custodian`
/// * `sale`
///
/// Mutable
/// * `buyer`
/// * `owner` (signer)
///
/// NOTE: With `claim_refunds`, remaining accounts are passed in
/// depending on however many accepted tokens there are for a given sale.
#[derive(Accounts)]
pub struct ClaimRefunds<'info> {
    #[account(
        seeds = [
            SEED_PREFIX_CUSTODIAN.as_bytes(),
        ],
        bump,
    )]
    pub custodian: Account<'info, Custodian>,

    #[account(
        seeds = [
            SEED_PREFIX_SALE.as_bytes(),
            &sale.id,
        ],
        bump,
    )]
    pub sale: Account<'info, Sale>,

    #[account(
        mut,
        seeds = [
            SEED_PREFIX_BUYER.as_bytes(),
            &sale.id,
            &owner.key().as_ref(),
        ],
        bump,
    )]
    pub buyer: Account<'info, Buyer>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
