use anyhow::Result;
use shared::types::ProofOfReasoning;
use std::collections::HashMap;
use tracing::info;

pub struct ConsensusEngine {
    proofs: HashMap<String, ProofOfReasoning>,
    validators: Vec<String>,
    required_confirmations: u32,
}

impl ConsensusEngine {
    pub fn new(required_confirmations: u32) -> Self {
        Self {
            proofs: HashMap::new(),
            validators: Vec::new(),
            required_confirmations,
        }
    }
    
    pub fn add_validator(&mut self, validator: String) {
        self.validators.push(validator);
    }
    
    pub fn submit_proof(&mut self, proof: ProofOfReasoning) -> Result<()> {
        info!("📥 Received proof from: {}", proof.prover);
        self.proofs.insert(proof.input_hash.clone(), proof);
        Ok(())
    }
    
    pub fn validate_proof(&mut self, proof_id: &str, validator: &str) -> Result<bool> {
        if let Some(proof) = self.proofs.get_mut(proof_id) {
            proof.confirmations += 1;
            
            info!("✅ Proof {} validated by {}", proof_id, validator);
            info!("   Confirmations: {}/{}", proof.confirmations, self.required_confirmations);
            
            if proof.confirmations >= self.required_confirmations {
                proof.verified = true;
                info!("🎉 Proof {} reached consensus!", proof_id);
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    pub fn get_verified_proofs(&self) -> Vec<&ProofOfReasoning> {
        self.proofs
            .values()
            .filter(|p| p.verified)
            .collect()
    }
    
    pub fn calculate_rewards(&self, proof_id: &str) -> Result<HashMap<String, u64>> {
        let mut rewards = HashMap::new();
        
        if let Some(proof) = self.proofs.get(proof_id) {
            if proof.verified {
                // Prover gets 70% of reward
                rewards.insert(proof.prover.clone(), 700);
                
                // Validators share 30%
                let validator_reward = 300 / proof.confirmations as u64;
                for validator in &self.validators {
                    rewards.insert(validator.clone(), validator_reward);
                }
            }
        }
        
        Ok(rewards)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_consensus() {
        let mut engine = ConsensusEngine::new(3);
        
        engine.add_validator("validator1".to_string());
        engine.add_validator("validator2".to_string());
        engine.add_validator("validator3".to_string());
        
        let proof = ProofOfReasoning {
            input_hash: "0xabc".to_string(),
            output_hash: "0xdef".to_string(),
            prover: "prover1".to_string(),
            timestamp: 1234567890,
            verified: false,
            confirmations: 0,
        };
        
        engine.submit_proof(proof).unwrap();
        
        assert!(!engine.validate_proof("0xabc", "validator1").unwrap());
        assert!(!engine.validate_proof("0xabc", "validator2").unwrap());
        assert!(engine.validate_proof("0xabc", "validator3").unwrap());
    }
}
