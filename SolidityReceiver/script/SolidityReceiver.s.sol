// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

import {Script, console} from "forge-std/Script.sol";

contract SolidityReceiverScript is Script {
    function setUp() public {}

    function run() public {
        vm.broadcast();
    }
}
