use alloy_sol_types::SolType;
use clap::Parser;
use dice_proof_lib::DiceGameValuesStruct;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

/// RISC-V ELF file for the Dice Game proof program.
pub const DICE_GAME_ELF: &[u8] = include_elf!("dice_proof_program");

/// Command line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long, default_value = "0")]
    start_time: u32,

    #[clap(long, default_value = "0")]
    end_time: u32,

    #[clap(long, default_value = "0")]
    duration: u32,

    #[clap(long, default_value = "0000000000000000000000000000000000000000000000000000000000000000")]
    task_hash: String,

    #[clap(long, default_value = "0")]
    player_score: u32,

    #[clap(long, default_value = "0")]
    bot_score: u32,

    #[clap(long, default_value = "game_000000")]
    game_id: String,

    #[clap(long, default_value = "player")]
    player: String,
}

fn main() {
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    let client = ProverClient::from_env();

    let task_hash_bytes: [u8; 32] = hex::decode(&args.task_hash)
        .expect("Invalid task hash hex string")
        .try_into()
        .expect("Invalid task hash length");
    let _task_hash_bytes = task_hash_bytes; // Fix unused variable warning

    let game_id_hash = create_hash_from_str(&args.game_id);
    let player_hash = create_hash_from_str(&args.player);
    let timestamp = args.end_time;
    let player_score = if args.player_score > 0 { args.player_score } else { args.duration };

    let mut stdin = SP1Stdin::new();
    stdin.write(&player_score);
    stdin.write(&args.bot_score);
    stdin.write(&timestamp);
    stdin.write(&game_id_hash);
    stdin.write(&player_hash);

    println!("Dice Game Data: Player Score = {}, Bot Score = {}, Game ID = {}, Player = {}",
             player_score, args.bot_score, args.game_id, args.player);

    if args.execute {
        let (output, report) = client.execute(DICE_GAME_ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");
        let decoded = DiceGameValuesStruct::abi_decode(output.as_slice(), true).unwrap();
        let DiceGameValuesStruct { gameId: _, playerScore, botScore, timestamp, isValid, playerHash: _ } = decoded;
        
        println!("Game Verification Result:");
        println!("Game ID: {}", args.game_id);
        println!("Player: {}", args.player);
        println!("Player Score: {}", playerScore);
        println!("Bot Score: {}", botScore);
        println!("Timestamp: {}", timestamp);
        println!("Verification Status: {}", if isValid == 1 { "SUCCESS" } else { "FAILED" });
        println!("Number of instructions executed: {}", report.total_instruction_count());
    } else {
        let (pk, vk) = client.setup(DICE_GAME_ELF);
        let proof = client.prove(&pk, &stdin).run().expect("proof generation failed");
        println!("Proof successfully generated!");
        client.verify(&proof, &vk).expect("proof verification failed");
        println!("Proof successfully verified!");
        let proof_path = format!("dice_game_proof_{}.bin", args.game_id);
        proof.save(&proof_path).expect("failed to save proof");
        println!("Proof saved to file: {}", proof_path);
    }
}

fn create_hash_from_str(input: &str) -> [u8; 32] {
    let mut hash = [0u8; 32];
    let input_bytes = input.as_bytes();
    for (i, &byte) in input_bytes.iter().enumerate().take(32) {
        hash[i] = byte;
    }
    hash
}
