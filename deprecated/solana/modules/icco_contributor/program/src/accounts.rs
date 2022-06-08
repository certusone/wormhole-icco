use crate::types::*;
use bridge::{
    accounts::BridgeData,
    api::ForeignAddress,
};
use solana_program::pubkey::Pubkey;
use solitaire::{
    processors::seeded::Seeded,
    *,
};

pub type AuthoritySigner<'b> = Derive<Info<'b>, "authority_signer">;
pub type CustodySigner<'b> = Derive<Info<'b>, "custody_signer">;
pub type MintSigner<'b> = Derive<Info<'b>, "mint_signer">;

pub type CoreBridge<'a, const STATE: AccountState> = Data<'a, BridgeData, { STATE }>;

pub type EmitterAccount<'b> = Derive<Info<'b>, "emitter">;

///-------------------------------------------------------------------
/// Cntributor Config.
pub type ConfigAccount<'b, const STATE: AccountState> =
    Derive<Data<'b, Config, { STATE }>, "config">;


///-------------------------------------------------------------------
/// icco Sale state. PDA <= "state", SaleId
pub type SaleStateAccount<'b, const STATE: AccountState> =
    Data<'b, SaleState, { STATE }>;

pub struct SaleStateDerivationData {
    pub sale_id: u128,
}

impl<'b, const STATE: AccountState> Seeded<&SaleStateDerivationData>
    for SaleStateAccount<'b, { STATE }>
{
    fn seeds(accs: &SaleStateDerivationData) -> Vec<Vec<u8>> {
        vec![
            String::from("state").as_bytes().to_vec(),
            accs.sale_id.to_be_bytes().to_vec(),
        ]
    }
}


///-------------------------------------------------------------------
/// Custody Account.
pub type CustodyAccount<'b, const STATE: AccountState> = Data<'b, SplAccount, { STATE }>;

pub struct CustodyAccountDerivationData {
    pub mint: Pubkey,
}

impl<'b, const STATE: AccountState> Seeded<&CustodyAccountDerivationData>
    for CustodyAccount<'b, { STATE }>
{
    fn seeds(accs: &CustodyAccountDerivationData) -> Vec<Vec<u8>> {
        vec![accs.mint.to_bytes().to_vec()]
    }
}


///-------------------------------------------------------------------
/// Registered chain endpoint
pub type Endpoint<'b, const STATE: AccountState> = Data<'b, EndpointRegistration, { STATE }>;

pub struct EndpointDerivationData {
    pub emitter_chain: u16,
    pub emitter_address: ForeignAddress,
}

/// Seeded implementation based on an incoming VAA
impl<'b, const STATE: AccountState> Seeded<&EndpointDerivationData> for Endpoint<'b, { STATE }> {
    fn seeds(data: &EndpointDerivationData) -> Vec<Vec<u8>> {
        vec![
            data.emitter_chain.to_be_bytes().to_vec(),
            data.emitter_address.to_vec(),
        ]
    }
}

///-------------------------------------------------------------------
/// Token metadata.
pub type SplTokenMeta<'b> = Info<'b>;

pub struct SplTokenMetaDerivationData {
    pub mint: Pubkey,
}

impl<'b> Seeded<&SplTokenMetaDerivationData> for SplTokenMeta<'b> {
    fn seeds(data: &SplTokenMetaDerivationData) -> Vec<Vec<u8>> {
        vec![
            "metadata".as_bytes().to_vec(),
            spl_token_metadata::id().as_ref().to_vec(),     // Why ID is needed?
            data.mint.as_ref().to_vec(),
        ]
    }
}
