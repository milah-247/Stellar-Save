# Demo Script: 3-Member ROSCA on Testnet

This walkthrough creates a 3-member ROSCA group, has each member join and contribute, and watches the payout rotate.

## Prerequisites

```bash
# Install Stellar CLI
cargo install stellar-cli --features opt

# Create identities for the demo
stellar keys generate alice --network testnet
stellar keys generate bob   --network testnet
stellar keys generate carol --network testnet

# Fund each account via Friendbot
stellar keys fund alice --network testnet
stellar keys fund bob   --network testnet
stellar keys fund carol --network testnet

# Deploy the contract (outputs CONTRACT_ID)
./scripts/deploy_testnet.sh
export CONTRACT="<CONTRACT_ID from above>"
```

## Step 1 — Create a Group

```bash
# 10 XLM per cycle, 100-ledger cycles, 3 members max
stellar contract invoke \
  --network testnet --source alice \
  --id $CONTRACT \
  -- create_group \
  --contribution_amount 100000000 \
  --cycle_duration 100 \
  --max_members 3
# → 0   (group ID)
export GROUP=0
```

## Step 2 — Members Join

```bash
stellar contract invoke --network testnet --source alice --id $CONTRACT \
  -- join_group --group_id $GROUP --member $(stellar keys address alice)

stellar contract invoke --network testnet --source bob --id $CONTRACT \
  -- join_group --group_id $GROUP --member $(stellar keys address bob)

stellar contract invoke --network testnet --source carol --id $CONTRACT \
  -- join_group --group_id $GROUP --member $(stellar keys address carol)
```

After Carol joins the group is full and cycle 1 begins automatically.

```bash
stellar contract invoke --network testnet --source alice --id $CONTRACT \
  -- get_group --group_id $GROUP
# → current_cycle: 1, payout_index: 0
```

## Step 3 — Cycle 1 Contributions → Alice Receives Payout

```bash
XLM_TOKEN="CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"  # testnet XLM

stellar contract invoke --network testnet --source alice --id $CONTRACT \
  -- contribute --group_id $GROUP --member $(stellar keys address alice) --token $XLM_TOKEN

stellar contract invoke --network testnet --source bob --id $CONTRACT \
  -- contribute --group_id $GROUP --member $(stellar keys address bob) --token $XLM_TOKEN

stellar contract invoke --network testnet --source carol --id $CONTRACT \
  -- contribute --group_id $GROUP --member $(stellar keys address carol) --token $XLM_TOKEN
```

After Carol's contribution all 3 are in — payout fires automatically.
Alice receives **30 XLM** (3 × 10 XLM). Cycle advances to 2.

## Step 4 — Check Contribution Status

```bash
stellar contract invoke --network testnet --source alice --id $CONTRACT \
  -- get_contribution_status --group_id $GROUP --cycle_number 1
# → [true, true, true]
```

## Step 5 — Cycle 2 (Bob receives) & Cycle 3 (Carol receives)

Repeat Step 3 twice more. After the third round:

```bash
stellar contract invoke --network testnet --source alice --id $CONTRACT \
  -- is_complete --group_id $GROUP
# → true
```

Each member contributed 30 XLM total and received 30 XLM once — net zero, exactly as intended.

## Verification on Stellar Expert

All transactions are visible at:
`https://stellar.expert/explorer/testnet/contract/$CONTRACT`
