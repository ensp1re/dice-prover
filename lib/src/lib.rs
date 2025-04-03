use alloy_sol_types::sol;

sol! {
    /// Structure containing dice game results that can be easily deserialized by Solidity.
    struct DiceGameValuesStruct {
        bytes32 gameId;     // Game ID
        uint32 playerScore; // Player's score
        uint32 botScore;    // Bot's score 
        uint32 timestamp;   // When the game was played
        uint32 isValid;     // Valid game? (1=yes, 0=no)
        bytes32 playerHash; // Player username hash
    }
}

/// Verifies the dice game results
pub fn verify_dice_game(player_score: u32, bot_score: u32, _timestamp: u32, game_id: &[u8; 32], player_hash: &[u8; 32]) -> bool {
    // Basic validation - scores must be between 2 and 12 (two dice)
    if player_score < 2 || player_score > 12 || bot_score < 2 || bot_score > 12 {
        return false;
    }


    // Ensure game ID and player hash are not empty (basic validity check)
    if game_id.iter().all(|&b| b == 0) || player_hash.iter().all(|&b| b == 0) {
        return false;
    }

    // Additional validation can be added here
    
    // Return true if all checks pass
    true
}
