// SPDX-License-Identifier: MIT

pragma solidity ^0.8.21;

contract SolidityReceiver {
    bytes4 private immutable _retval;

    event Received(address operator, address from, uint256 tokenId);

    error CustomError(bytes4);

    constructor() {
        _retval = 0x150b7a02;
    }

    function handle(address operator, address from, uint256 tokenId) public returns (bytes4) {
        emit Received(operator, from, tokenId);
        return _retval;
    }
}
