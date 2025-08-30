# Solana Prediction Market (Anchor)

This is a simple prediction market smart contract built using [Anchor](https://book.anchor-lang.com/).  
It allows users to bet SOL on whether the **SOL/USD price** will go **Up** or **Down** in a given round.  
The contract integrates with **Pyth price feeds** (can be extended to Switchboard).

---

## ✨ Features
- Rounds last for a fixed duration (default: 5 minutes).
- Users place bets on **Up** or **Down** with SOL.
- At round end, the contract checks the oracle price and determines winners.
- Winners can claim rewards.

---

## 📦 Project Structure
```
programs/
 └── prediction_market/
     ├── src/
     │   ├── lib.rs          # Program entry
     │   ├── contexts.rs     # Anchor Contexts
     │   ├── instructions.rs # Instruction handlers
     │   ├── states.rs       # Round / Bet account structs
     │   ├── errors.rs       # Custom errors
tests/
 └── prediction.test.ts      # Mocha/Chai tests with Anchor
```
---

## ⚡ Quick Start

### 1️⃣ Install dependencies
```bash
yarn install
```

### 2️⃣ Build the program
```bash
anchor build
```

### 3️⃣ Deploy locally
```bash
anchor deploy
```

### 4️⃣ Run tests
```bash
anchor test
```

## 📜 License
MIT
