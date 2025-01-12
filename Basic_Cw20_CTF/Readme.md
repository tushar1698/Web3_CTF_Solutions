# Basic CW20 with Fee - CTF Challenge Explanation

Welcome to the **CW20 CTF Challenge**! This document will guide you through the problem statement, vulnerabilities, and how solvers can approach the challenge.
## Challenge Level --> Beginner

## Required Experience
- Good Knowledge of `Rust`
- Basic Understanding of [CosmWasm](https://docs.cosmwasm.com/) and [Cw20 token standard](https://github.com/CosmWasm/cw-plus/blob/main/packages/cw20/README.md) 

## Overview

This CTF revolves around a slightly modified implementation of a CW20 token contract with added functionality for fee deduction during transfers. While the contract appears secure at first glance, subtle vulnerabilities have been intentionally introduced to challenge solvers.

---

## Getting Started
1. Clone the Repo:
```bash
git clone https://github.com/tushar1698/Web3_CTF_Solutions/tree/main/Basic_Cw20_CTF.git
cd Basic_CW20_with_Fee_CTF
```

2. Install Dependencies
```bash
cargo install cosmwasm-cli
```
3. Build and Test
```bash
cargo build
cargo test
```
## Key Features

1. **Transfer with Fees**:
   - Transfers include a fee deduction mechanism, where a small percentage of the transferred amount is sent to a fee collector.

2. **Allowance Management**:
   - Standard CW20 allowance functions for third-party spending are implemented.

3. **Mint and Burn**:
   - Owners can mint tokens within a maximum supply limit and burn tokens from their account.

4. **Intentional Vulnerabilities**:
   -  Bugs have been introduced to create security challenges.


---

## Solver's Objectives

1. **Identify Vulnerabilities**:
   - Analyze the contract to identify and understand the intentional vulnerabilities.

2. **Exploit the Vulnerabilities**:
   - Write test cases or execute transactions to demonstrate how the vulnerabilities can be exploited.

3. **Document Findings**:
   - Provide a clear explanation of the issues and their potential impact.

---

## Exploiting the Bugs

The following test cases in the `tests/unit_tests.rs` file will help solvers verify their understanding of the vulnerabilities
You just need to add your tests at the bottom and run `cargo test -- --nocapture`

## Disclaimer

This CTF is for educational purposes only. Please refrain from using these techniques on real-world contracts without proper authorization.

Good luck and happy hacking!
