use sp1_sdk::{include_elf, HashableKey, ProverClient};

/// RISC-V ELF file for the DICE Score proof program.
pub const DICE_PROOF_ELF: &[u8] = include_elf!("dice_proof_program");

fn main() {
    // Setup prover client
    let client = ProverClient::from_env();
    
    // Get verification key for the program
    let (_, vk) = client.setup(DICE_PROOF_ELF);
    
    // Print verification key
    println!("DICE Score Program VKey: {}", vk.bytes32());
}
