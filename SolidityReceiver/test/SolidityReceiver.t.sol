// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

import {Test, console} from "forge-std/Test.sol";
import {SolidityReceiver} from "../src/SolidityReceiver.sol";

contract SolidityReceiverTest is Test {
    SolidityReceiver public receiver;

    function setUp() public {
        receiver = new SolidityReceiver();
    }

    function test_Handle() public pure {
        assertEq(uint8(1 + 1), uint8(2));
    }

    function testFuzz_Handle(uint256 tokenId, bytes memory data) public {
        receiver.handle(tokenId, data);
        assertEq(uint8(1 + 1), uint8(2));
    }
}
