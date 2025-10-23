#[cfg(test)]
mod tests {
    use por_consensus::ConsensusEngine;
    use shared::types::ProofOfReasoning;

    #[test]
    fn test_proof_submission() {
        let mut engine = ConsensusEngine::new(3);
        
        let proof = ProofOfReasoning {
            input_hash: "0xabc123".to_string(),
            output_hash: "0xdef456".to_string(),
            prover: "validator1".to_string(),
            timestamp: 1234567890,
            verified: false,
            confirmations: 0,
        };
        
        assert!(engine.submit_proof(proof).is_ok());
    }

    #[test]
    fn test_consensus_reached() {
        let mut engine = ConsensusEngine::new(3);
        
        engine.add_validator("validator1".to_string());
        engine.add_validator("validator2".to_string());
        engine.add_validator("validator3".to_string());
        
        let proof = ProofOfReasoning {
            input_hash: "0xabc123".to_string(),
            output_hash: "0xdef456".to_string(),
            prover: "prover1".to_string(),
            timestamp: 1234567890,
            verified: false,
            confirmations: 0,
        };
        
        engine.submit_proof(proof).unwrap();
        
        // First two validations should not reach consensus
        assert!(!engine.validate_proof("0xabc123", "validator1").unwrap());
        assert!(!engine.validate_proof("0xabc123", "validator2").unwrap());
        
        // Third validation should reach consensus
        assert!(engine.validate_proof("0xabc123", "validator3").unwrap());
    }

    #[test]
    fn test_reward_calculation() {
        let mut engine = ConsensusEngine::new(3);
        
        engine.add_validator("validator1".to_string());
        engine.add_validator("validator2".to_string());
        engine.add_validator("validator3".to_string());
        
        let proof = ProofOfReasoning {
            input_hash: "0xabc123".to_string(),
            output_hash: "0xdef456".to_string(),
            prover: "prover1".to_string(),
            timestamp: 1234567890,
            verified: false,
            confirmations: 3,
        };
        
        engine.submit_proof(proof).unwrap();
        
        let rewards = engine.calculate_rewards("0xabc123").unwrap();
        
        // Prover should get 70% (700)
        assert_eq!(rewards.get("prover1"), Some(&700));
        
        // Each validator should get share of 30%
        assert!(rewards.get("validator1").is_some());
    }

    #[test]
    fn test_verified_proofs() {
        let mut engine = ConsensusEngine::new(2);
        
        engine.add_validator("validator1".to_string());
        engine.add_validator("validator2".to_string());
        
        let proof = ProofOfReasoning {
            input_hash: "0xabc123".to_string(),
            output_hash: "0xdef456".to_string(),
            prover: "prover1".to_string(),
            timestamp: 1234567890,
            verified: false,
            confirmations: 0,
        };
        
        engine.submit_proof(proof).unwrap();
        
        engine.validate_proof("0xabc123", "validator1").unwrap();
        engine.validate_proof("0xabc123", "validator2").unwrap();
        
        let verified = engine.get_verified_proofs();
        assert_eq!(verified.len(), 1);
        assert!(verified[0].verified);
    }
}
