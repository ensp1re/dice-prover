//! SP1 proof program for the Dice Game.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::{SolType, private::FixedBytes};
use dice_proof_lib::{verify_dice_game, DiceGameValuesStruct};

pub fn main() {
    // Read input data
    let player_score = sp1_zkvm::io::read::<u32>();
    let bot_score = sp1_zkvm::io::read::<u32>();
    let timestamp = sp1_zkvm::io::read::<u32>();
    let game_id_hash = sp1_zkvm::io::read::<[u8; 32]>();
    let player_hash = sp1_zkvm::io::read::<[u8; 32]>();

    // Verification process
let is_valid = verify_dice_game(
    player_score,
    bot_score,
    timestamp,
    &game_id_hash,  
    &player_hash    
);

    // Create public values
    let public_values = DiceGameValuesStruct {
        gameId: FixedBytes(game_id_hash),
        playerScore: player_score,
        botScore: bot_score,
        timestamp: timestamp,
        isValid: if is_valid { 1 } else { 0 },
        playerHash: FixedBytes(player_hash),
    };

    // Debug outputs
    println!("Dice Game verification:");
    println!("Player Score: {}", player_score);
    println!("Bot Score: {}", bot_score);
    println!("Timestamp: {}", timestamp);
    println!("Verification Result: {}", if is_valid { "SUCCESS" } else { "FAILED" });

    // Encode results and provide as output
    let bytes = DiceGameValuesStruct::abi_encode(&public_values);
    sp1_zkvm::io::commit_slice(&bytes);
}
