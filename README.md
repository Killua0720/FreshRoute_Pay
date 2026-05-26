Contract ID: CCAYFDAK5GEGEVTDSQ2F7PGEPDT2YCPXEJZSSOYEFY2B2J42M6MM6AKR
Link: https://stellar.expert/explorer/testnet/contract/CCAYFDAK5GEGEVTDSQ2F7PGEPDT2YCPXEJZSSOYEFY2B2J42M6MM6AKR

<img width="1920" height="956" alt="Screenshot 2026-05-26 115805" src="https://github.com/user-attachments/assets/71c91a08-b52d-4409-af44-6b75b23beac5" />



# 🥬 FreshRoute Pay

> **On-chain produce payment escrow for Philippine farmers and wet market buyers — powered by Stellar Soroban.**

---

## 🧩 The Problem

A vegetable consolidator in Nueva Ecija, Philippines buys lettuce and pechay from 18 smallholder farmers every morning using handwritten delivery logs and delayed bank transfers. Farmers often wait 2–3 days to receive payment after successful delivery, forcing them to borrow money for fertilizer, fuel, and labor while disputes over missing sacks and incorrect weights remain unresolved. Small agricultural suppliers in the Philippines lose millions yearly from delayed settlement and unverifiable produce transactions.

## ✅ The Solution

FreshRoute Pay lets a buyer lock USDC payment into a Soroban escrow contract before produce delivery begins. Farmers deliver vegetables and the buyer confirms receipt directly on-chain using a mobile app. Once confirmed, the contract instantly releases payment to the farmer’s Stellar wallet, creating an immutable payment and delivery audit trail that both parties can verify publicly.

**Why Stellar?** 5-second finality, near-zero transaction fees, and native stablecoin support make Stellar ideal for high-frequency agricultural payments where traditional banking delays directly impact farmer income and harvest operations.

---

## 🌟 Stellar Features Used

| Feature | Usage |
|---|---|
| Soroban Smart Contracts | Escrow logic for delivery-confirmed payouts |
| USDC on Stellar | Instant stablecoin settlement for produce payments |
| Stellar Addresses | Buyer and farmer identity verification |
| Trustlines | Secure USDC acceptance and payment routing |
| Testnet XLM | Gas fees for on-chain demo transactions |

---

## 🎯 Target Users

- **Farmers** — smallholder vegetable farmers in Nueva Ecija, Pampanga, and Tarlac
- **Produce Consolidators** — SME buyers supplying Metro Manila wet markets
- **Wet Market Retailers** — stall operators in Balintawak, Divisoria, and Pasig markets
- **Agricultural Cooperatives** — organizations coordinating bulk produce deliveries and payouts

---

## 🔁 Core MVP Transaction Flow

```text
Buyer → initialize(buyer, farmer, amount)
         ↓ [escrow agreement stored on-chain]

Farmer → deliver produce
          ↓ [physical delivery completed]

Buyer → confirm_delivery(buyer)
         ↓ [status updated: Delivered]

Contract → release_payment()
            ↓ [payment released to farmer wallet]

Farmer → payment_status()
          ← [reads payment confirmation]
```

**Demo time: ~90 seconds** — initialize escrow, confirm produce delivery, release payment, then verify payment status.

---

## 🏆 Why This Wins

FreshRoute Pay solves a real-world agricultural cash flow problem affecting thousands of Filipino farmers every harvest cycle. The project demonstrates real financial coordination using Stellar instead of speculative crypto use cases, while showcasing a fast and visually understandable Soroban escrow flow judges can demo live in under two minutes.

---

## 🚀 Prerequisites

- Rust: `rustup target add wasm32-unknown-unknown`
- Soroban CLI: `cargo install --locked stellar-cli --features opt`
- Stellar testnet account funded via Friendbot

---

## 🔨 Build

```bash
# Build the Wasm contract
soroban contract build

# Output:
# target/wasm32-unknown-unknown/release/fresh_route_pay.wasm
```

---

## 🧪 Test

```bash
# Run all 5 unit tests
cargo test

# Run with logs
cargo test --features testutils -- --nocapture
```

---

## 🌐 Deploy to Testnet

```bash
# 1. Configure your testnet identity
stellar keys generate --global alice --network testnet
stellar keys fund alice --network testnet

# 2. Deploy the contract
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/fresh_route_pay.wasm \
  --source alice \
  --network testnet

# Save the returned CONTRACT_ID
```

---

## 🖥️ Sample CLI Invocations

```bash
# Step 1: Initialize escrow agreement
stellar contract invoke \
  --id $CONTRACT_ID \
  --source buyer_key \
  --network testnet \
  -- initialize \
  --buyer $(stellar keys address buyer_key) \
  --farmer $(stellar keys address farmer_key) \
  --amount 500

# Step 2: Buyer confirms produce delivery
stellar contract invoke \
  --id $CONTRACT_ID \
  --source buyer_key \
  --network testnet \
  -- confirm_delivery \
  --buyer $(stellar keys address buyer_key)

# Step 3: Release payment
stellar contract invoke \
  --id $CONTRACT_ID \
  --source buyer_key \
  --network testnet \
  -- release_payment

# Step 4: Check payment status
stellar contract invoke \
  --id $CONTRACT_ID \
  --network testnet \
  -- payment_status
```

---

## 🗺️ Vision & Purpose

FreshRoute Pay is Phase 1 of a larger agricultural settlement and logistics network for Southeast Asia. Phase 2 introduces cooperative financing and produce-backed reward tokens for farmers with consistent delivery performance. Future integrations include local agriculture cooperatives, supplier credit systems, and QR-based produce tracking across public wet markets.

The long-term goal is to eliminate delayed agricultural payments in Philippine food supply chains by giving every produce transaction an instant, transparent, and verifiable on-chain settlement record powered by Stellar.

---

## 📄 License

MIT © 2026 FreshRoute Pay Contributors
