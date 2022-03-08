import { ethers } from "ethers";
import { parseUnits } from "ethers/lib/utils";
import {
  ChainId,
  IWETH__factory,
  hexToNativeString,
  hexToUint8Array,
  importCoreWasm,
  nativeToHexString,
  uint8ArrayToHex,
  ERC20__factory,
} from "..";

export function nativeToUint8Array(
  address: string,
  chainId: ChainId
): Uint8Array {
  return hexToUint8Array(nativeToHexString(address, chainId) || "");
}

export async function wrapEth(
  wallet: ethers.Wallet,
  wethAddress: string,
  amount: string
): Promise<void> {
  const weth = IWETH__factory.connect(wethAddress, wallet);
  await weth.deposit({
    value: parseUnits(amount),
  });
}

export async function getCurrentBlock(
  provider: ethers.providers.Provider
): Promise<ethers.providers.Block> {
  const currentBlockNumber = await provider.getBlockNumber();
  return provider.getBlock(currentBlockNumber);
}

export async function sleepFor(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export async function getErc20Balance(
  provider: ethers.providers.Provider,
  tokenAddress: string,
  walletAddress: string
): Promise<ethers.BigNumber> {
  const token = ERC20__factory.connect(tokenAddress, provider);
  return token.balanceOf(walletAddress);
}
