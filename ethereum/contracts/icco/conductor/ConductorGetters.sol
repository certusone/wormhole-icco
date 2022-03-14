// contracts/Getters.sol
// SPDX-License-Identifier: Apache 2

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import "../../interfaces/IWormhole.sol";
import "../../bridge/BridgeImplementation.sol";

import "./ConductorState.sol";

contract ConductorGetters is ConductorState {
    function governanceActionIsConsumed(bytes32 hash) public view returns (bool) {
        return _state.consumedGovernanceActions[hash];
    }

    function isInitialized(address impl) public view returns (bool) {
        return _state.initializedImplementations[impl];
    }

    function wormhole() public view returns (IWormhole) {
        return IWormhole(_state.provider.wormhole);
    }

    function tokenBridge() public view returns (BridgeImplementation) {
        return BridgeImplementation(payable(_state.provider.tokenBridge));
    }

    function chainId() public view returns (uint16){
        return _state.provider.chainId;
    }

    function governanceChainId() public view returns (uint16){
        return _state.provider.governanceChainId;
    }

    function governanceContract() public view returns (bytes32){
        return _state.provider.governanceContract;
    }

    function contributorContracts(uint16 chainId_) public view returns (bytes32){
        return _state.contributorImplementations[chainId_];
    }

    function sales(uint saleId_) public view returns (ConductorStructs.Sale memory sale){
        return _state.sales[saleId_];
    }

    function getSaleStatus(uint saleId_) public view returns (bool isSealed, bool isAborted){
        return (
            _state.sales[saleId_].isSealed,
            _state.sales[saleId_].isAborted
        );
    }

    function getNextSaleId() public view returns (uint){
        return _state.nextSaleId;
    }

    function saleContributionIsCollected(uint saleId_, uint tokenIndex) public view returns (bool){
        return _state.sales[saleId_].contributionsCollected[tokenIndex];
    }

    function saleContributions(uint saleId_) public view returns (uint[] memory){
        return _state.sales[saleId_].contributions;
    }
}