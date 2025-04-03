# Dice Game

Dice Game is a web application that allows users to play a simple dice game, where they can roll dice and bet on the outcome.

## Technologies
- **Backend**: Node.js, Express
- **ZK Proof**: SP1 (Succinct Labs)
- **Programming Language**: Rust, JavaScript

## Local Setup for Real SP1 Proofs

### Requirements
- **Node.js** (v14+)
- **Rust** (1.79+)
- **SP1 Toolchain**

### Installation Steps
1. Clone this repository:

    ```bash
    git clone https://github.com/ensp1re/dice.git
    cd dice-game
    ```

2. Install backend dependencies:

    ```bash
    cd backend
    npm install
    ```

3. Install the SP1 toolchain (if not already installed):

    ```bash
    curl -L https://sp1up.succinct.xyz | bash
    sp1up
    ```

4. Build the SP1 program:

    ```bash
    cd ../program
    cargo prove build
    ```

5. Start the backend server:

    ```bash
    cd ../backend
    node server.js
    ```


## Usage
1. Place a bet on the dice roll outcome.
2. Click the "Roll Dice" button to initiate the dice roll.
3. View the result of your roll and check the cryptographic proof for fairness.
4. Share the result with others.

## Frontend
For the frontend of the Dice game, refer to the Dice game repository here: [Dice Game Frontend](https://github.com/ensp1re/Dice).

---

> For any issues or contributions, please feel free to open an issue or pull request.
