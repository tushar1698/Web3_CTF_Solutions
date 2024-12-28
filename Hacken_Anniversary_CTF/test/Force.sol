// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

contract Force {
    constructor(address _beneficiary) payable {
        // https://eips.ethereum.org/EIPS/eip-6780
        // SELFDESTRUCT transfers the entire account balance to the target address.
        selfdestruct(payable(_beneficiary));
    }
}