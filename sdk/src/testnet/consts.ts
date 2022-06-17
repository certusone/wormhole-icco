const fs = require("fs");
import { web3 } from "@project-serum/anchor";
import { ChainId, CHAIN_ID_ETH, CHAIN_ID_AVAX, CHAIN_ID_SOLANA } from "@certusone/wormhole-sdk";

export const WORMHOLE_ADDRESSES = {
  guardianRpc: ["https://wormhole-v2-testnet-api.certus.one"],
  solana_testnet: {
    wormhole: "3u8hJUVTA4jH1wYAyUur7FFZVQ8H635K3tSHHF4ssjQ5",
    tokenBridge: "DZnkkTmCiFWfYTfT41X3Rd1kDgozqzxWaHqsw6W4x2oe",
    chainId: 1,
  },
  goerli: {
    wormhole: "0x706abc4E45D419950511e474C7B9Ed348A4a716c",
    tokenBridge: "0xF890982f9310df57d00f659cf4fd87e65adEd8d7",
    chainId: 2,
  },
  fuji: {
    wormhole: "0x7bbcE28e64B3F8b84d876Ab298393c38ad7aac4C",
    tokenBridge: "0x61E44E506Ca5659E6c0bba9b678586fA2d729756",
    chainId: 6,
  },
  binance_testnet: {
    wormhole: "0x68605AD7b15c732a30b1BbC62BE8F2A509D74b4D",
    tokenBridge: "0x9dcF9D205C9De35334D646BeE44b2D2859712A09",
    chainId: 4,
  },
  mumbai: {
    wormhole: "0x0CBE91CF822c73C2315FB05100C2F714765d5c20",
    tokenBridge: "0x377D55a7928c046E18eEbb61977e714d2a76472a",
    chainId: 5,
  },
  fantom_testnet: {
    wormhole: "0x1BB3B4119b7BA9dfad76B0545fb3F531383c3bB7",
    tokenBridge: "0x599CEa2204B4FaECd584Ab1F2b6aCA137a0afbE8",
    chainId: 10,
  },
};

const REPO_ROOT = `${__dirname}/../../..`;
const TESTNET_CFG = `${REPO_ROOT}/sdk/cfg/testnet`;

export const TESTNET_ADDRESSES = JSON.parse(fs.readFileSync(`${REPO_ROOT}/testnet.json`, "utf8"));
export const SALE_CONFIG = JSON.parse(fs.readFileSync(`${TESTNET_CFG}/saleConfig.json`, "utf8"));
export const CONTRIBUTOR_INFO = JSON.parse(fs.readFileSync(`${TESTNET_CFG}/contributors.json`, "utf8"));
//export const SOLANA_IDL = JSON.parse(fs.readFileSync(`${__dirname}/../solana/anchor_contributor.json`, "utf8"));

// VAA fetching params
export const RETRY_TIMEOUT_SECONDS = 180;

// deployment info for the sale
export const SOLANA_RPC = SALE_CONFIG["initiatorWallet"]["solana_testnet"].rpc;
export const SOLANA_CORE_BRIDGE_ADDRESS = new web3.PublicKey(WORMHOLE_ADDRESSES.solana_testnet.wormhole);
export const CONDUCTOR_ADDRESS = TESTNET_ADDRESSES.conductorAddress;
export const CONDUCTOR_CHAIN_ID = TESTNET_ADDRESSES.conductorChain;
export const CONDUCTOR_NETWORK = SALE_CONFIG["conductorNetwork"];
export const KYC_AUTHORITY_KEY = SALE_CONFIG["authority"];
export const CONTRIBUTOR_NETWORKS: string[] = ["goerli", "fuji", "solana_devnet"];
export const CHAIN_ID_TO_NETWORK = new Map<ChainId, string>();
CHAIN_ID_TO_NETWORK.set(CHAIN_ID_ETH, CONTRIBUTOR_NETWORKS[0]);
CHAIN_ID_TO_NETWORK.set(CHAIN_ID_AVAX, CONTRIBUTOR_NETWORKS[1]);
CHAIN_ID_TO_NETWORK.set(CHAIN_ID_SOLANA, CONTRIBUTOR_NETWORKS[2]);
