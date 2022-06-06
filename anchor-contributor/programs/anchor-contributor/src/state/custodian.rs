use anchor_lang::prelude::*;

use crate::{
    constants::INDEX_SALE_ID,
    env::{CONDUCTOR_ADDRESS, CONDUCTOR_CHAIN},
    error::ContributorError,
    wormhole::{get_message_data, MessageData},
};

#[account]
#[derive(Default)]
pub struct Custodian {
    pub owner: Pubkey, // 32
    pub nonce: u32,    // 4
}

impl Custodian {
    pub const MAXIMUM_SIZE: usize = 32 + 4;

    pub fn conductor_chain() -> Result<u16> {
        let chain_id: u16 = match CONDUCTOR_CHAIN.to_string().parse() {
            Ok(v) => v,
            _ => return Result::Err(ContributorError::InvalidConductorChain.into()),
        };
        Ok(chain_id)
    }

    pub fn conductor_address() -> Result<[u8; 32]> {
        let mut addr = [0u8; 32];
        addr.copy_from_slice(&match hex::decode(CONDUCTOR_ADDRESS) {
            Ok(decoded) => decoded,
            _ => return Result::Err(ContributorError::InvalidConductorAddress.into()),
        });
        Ok(addr)
    }

    pub fn new(&mut self, owner: &Pubkey) -> Result<()> {
        self.owner = owner.clone(); // owner doesn't actually do anything
        self.nonce = 0;
        Ok(())
    }

    pub fn parse_and_verify_conductor_vaa<'info>(
        &self,
        vaa_acct: &AccountInfo<'info>,
        payload_type: u8,
    ) -> Result<MessageData> {
        let msg = get_message_data(&vaa_acct)?;
        require!(
            msg.emitter_chain == Custodian::conductor_chain()?,
            ContributorError::InvalidConductorChain
        );
        require!(
            msg.emitter_address == Custodian::conductor_address()?,
            ContributorError::InvalidConductorAddress
        );
        require!(
            msg.payload[0] == payload_type,
            ContributorError::InvalidVaaAction
        );
        Ok(msg)
    }

    pub fn get_sale_id_from_payload(payload: &[u8]) -> [u8; 32] {
        let mut sale_id = [0u8; 32];
        sale_id.copy_from_slice(&payload[INDEX_SALE_ID..INDEX_SALE_ID + 32]);
        sale_id
    }

    pub fn get_sale_id_from_vaa<'info>(vaa_acct: &AccountInfo<'info>) -> Result<[u8; 32]> {
        let msg = get_message_data(&vaa_acct)?;
        Ok(Custodian::get_sale_id_from_payload(&msg.payload))
    }

    pub fn parse_and_verify_conductor_vaa_and_sale<'info>(
        &self,
        vaa_acct: &AccountInfo<'info>,
        payload_type: u8,
        sale_id: [u8; 32],
    ) -> Result<MessageData> {
        let msg = self.parse_and_verify_conductor_vaa(vaa_acct, payload_type)?;
        require!(
            Custodian::get_sale_id_from_payload(&msg.payload) == sale_id,
            ContributorError::IncorrectSale,
        );
        Ok(msg)
    }
}
