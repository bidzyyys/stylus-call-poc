// SPDX-License-Identifier: MIT

pragma solidity ^0.8.21;

contract SolidityReceiver {
    bytes4 private immutable _retval;

    event Received(uint256 tokenId, bytes data);

    error CustomError(bytes4);

    constructor() {
        _retval = 0x150b7a02;
    }

    function handle(uint256 tokenId, bytes memory data) public returns (bytes4) {
        emit Received(tokenId, data);
        return _retval;
    }
}
