// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";

contract ProofOfReasoning is Ownable {
    
    struct ReasoningProof {
        bytes32 inputHash;
        bytes32 outputHash;
        address prover;
        uint256 timestamp;
        uint256 computationCost;
        bool verified;
        uint256 confirmations;
    }
    
    struct ValidationVote {
        address validator;
        bool approved;
        uint256 timestamp;
    }
    
    mapping(bytes32 => ReasoningProof) public proofs;
    mapping(bytes32 => ValidationVote[]) public votes;
    mapping(bytes32 => mapping(address => bool)) public hasVoted;
    
    uint256 public requiredConfirmations = 3;
    uint256 public rewardPerProof = 1 ether;
    
    event ProofSubmitted(bytes32 indexed proofId, address indexed prover);
    event ProofValidated(bytes32 indexed proofId, address indexed validator, bool approved);
    event ProofVerified(bytes32 indexed proofId, uint256 reward);
    
    constructor() Ownable(msg.sender) {}
    
    function submitProof(
        bytes32 inputHash,
        bytes32 outputHash,
        uint256 computationCost
    ) external returns (bytes32) {
        bytes32 proofId = keccak256(abi.encodePacked(
            inputHash,
            outputHash,
            msg.sender,
            block.timestamp
        ));
        
        require(proofs[proofId].prover == address(0), "Proof exists");
        
        proofs[proofId] = ReasoningProof({
            inputHash: inputHash,
            outputHash: outputHash,
            prover: msg.sender,
            timestamp: block.timestamp,
            computationCost: computationCost,
            verified: false,
            confirmations: 0
        });
        
        emit ProofSubmitted(proofId, msg.sender);
        return proofId;
    }
    
    function validateProof(bytes32 proofId, bool approved) external {
        require(proofs[proofId].prover != address(0), "Proof not found");
        require(!proofs[proofId].verified, "Already verified");
        require(!hasVoted[proofId][msg.sender], "Already voted");
        require(proofs[proofId].prover != msg.sender, "Cannot validate own proof");
        
        votes[proofId].push(ValidationVote({
            validator: msg.sender,
            approved: approved,
            timestamp: block.timestamp
        }));
        
        hasVoted[proofId][msg.sender] = true;
        
        if (approved) {
            proofs[proofId].confirmations++;
        }
        
        emit ProofValidated(proofId, msg.sender, approved);
        
        if (proofs[proofId].confirmations >= requiredConfirmations) {
            _finalizeProof(proofId);
        }
    }
    
    function _finalizeProof(bytes32 proofId) internal {
        ReasoningProof storage proof = proofs[proofId];
        proof.verified = true;
        
        (bool success, ) = proof.prover.call{value: rewardPerProof}("");
        require(success, "Reward transfer failed");
        
        emit ProofVerified(proofId, rewardPerProof);
    }
    
    function getProofStatus(bytes32 proofId) external view returns (
        address prover,
        bool verified,
        uint256 confirmations,
        uint256 totalVotes
    ) {
        ReasoningProof memory proof = proofs[proofId];
        return (
            proof.prover,
            proof.verified,
            proof.confirmations,
            votes[proofId].length
        );
    }
    
    function fundContract() external payable onlyOwner {}
    
    receive() external payable {}
}
