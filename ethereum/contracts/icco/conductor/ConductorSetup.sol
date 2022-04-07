// contracts/ConductorSetup.sol
// SPDX-License-Identifier: Apache 2

pragma solidity ^0.8.0;
pragma experimental ABIEncoderV2;

import "./ConductorGovernance.sol";

import "@openzeppelin/contracts/proxy/ERC1967/ERC1967Upgrade.sol";

contract ConductorSetup is ConductorSetters, ERC1967Upgrade {
    function setup(
        address implementation,
        uint16 chainId,
        address wormhole,
        address tokenBridge
    ) public {
        setOwner(_msgSender());

        setChainId(chainId);

        setWormhole(wormhole);

        setTokenBridge(tokenBridge);

        _upgradeTo(implementation);
    }
}
