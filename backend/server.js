const express = require('express');
const cors = require('cors');
const bodyParser = require('body-parser');
const { exec } = require('child_process');
const path = require('path');
const fs = require('fs');

const app = express();
const PORT = 4003;

// Middleware
app.use(cors());
app.use(bodyParser.json());

// Logging middleware
const logActivity = (req, res, next) => {
  const timestamp = new Date().toISOString();
  const logEntry = `[${timestamp}] ${req.method} ${req.originalUrl} from ${req.ip}\n`;
  fs.appendFile('access.log', logEntry, (err) => {
    if (err) console.error('Error writing to log file:', err);
  });
  console.log(logEntry.trim());
  next();
};

app.use(logActivity);

// Global error handler
const errorHandler = (err, req, res, next) => {
  console.error(`Error: ${err.message}`);
  res.status(500).json({ success: false, error: err.message });
};

// Health check
app.get('/health', (req, res) => {
  res.json({ status: 'ok', timestamp: new Date().toISOString() });
});

// Validate game data
const validateGameData = (data) => {
  return data.gameId && data.player && data.playerScore !== undefined && data.botScore !== undefined && data.winner;
};

// Generate proof for game result
app.post('/api/generate-proof', (req, res, next) => {
  const { gameId, player, playerScore, botScore, winner } = req.body;
  const timestamp = new Date().toISOString();

  console.log(`[${timestamp}] Received request:`, req.body);

  if (!validateGameData(req.body)) {
    console.error(`[${timestamp}] Validation failed: Missing required fields`);
    return res.status(400).json({ success: false, error: 'Missing required fields' });
  }

  const scriptPath = path.resolve(__dirname, '..', 'script');
  console.log(`[${timestamp}] Using script path: ${scriptPath}`);

  const command = `cargo run --bin prove --release -- --prove \
    --game-id ${gameId} \
    --player ${player} \
    --player-score ${playerScore} \
    --bot-score ${botScore}`;
  
  console.log(`[${timestamp}] Running command: ${command}`);

exec(command, { cwd: scriptPath, env: { ...process.env } }, (error, stdout, stderr) => {
    console.log(`[${timestamp}] Command output:`, stdout);
    if (stderr) console.error(`[${timestamp}] Command errors:`, stderr);

    if (error) {
      console.error(`[${timestamp}] Proof generation failed:`, error);
      return next(new Error(`Proof generation failed: ${stderr}`));
    }

    console.log(`[${timestamp}] Proof successfully generated`);
    res.json({ success: true, gameId, player, playerScore, botScore, winner, proofOutput: stdout });
  });
});

// Use global error handler
app.use(errorHandler);

// Start the server
app.listen(PORT, () => {
  console.log(`Server running at http://localhost:${PORT}`);
});
