# Architecture Overview

## What is Stellar-Save?

Stellar-Save implements a **Rotating Savings and Credit Association (ROSCA)** — known as *Ajo* or *Esusu* in West Africa — on the Stellar network using Soroban smart contracts.

Members pool equal contributions each cycle. One member receives the entire pool per cycle, rotating through all members until everyone has been paid once.

## High-Level Flow

```
[create_group] → group created, status=Active, cycle=0

[join_group × N] → when N == max_members, cycle advances to 1

     ┌───────────────────────────────┐
     │  Each Cycle                   │
     │                               │
     │  contribute() × max_members   │
     │      ↓ all contributed?       │
     │  execute_payout() (auto)      │
     │      ↓ payout to member[i]    │
     │  payout_index++, cycle++      │
     └───────────────────────────────┘
         (repeat max_members times)

[is_complete] → true when payout_index == max_members
```

## Contract Structure

```
contracts/stellar-save/
├── Cargo.toml
└── src/
    ├── lib.rs      — contract entry point, all public functions
    ├── types.rs    — Group, GroupStatus, DataKey
    ├── error.rs    — Error enum
    └── xlm.rs      — token transfer helper
```

## Key Design Decisions

**Single contract, multiple groups**
All ROSCA groups live inside one deployed contract. Groups are identified by an auto-incrementing `u64` ID.

**Automatic payout**
`contribute()` checks after each contribution whether all members have contributed for the current cycle. If so, it fires the payout immediately — no separate keeper/cron job required.

**Manual payout fallback**
`execute_payout()` is exposed publicly so anyone can trigger a payout manually once all contributions are confirmed, useful for off-chain tooling.

**Token agnostic**
The `token` address is passed per invocation rather than stored in the group, supporting any SEP-41 compatible asset. XLM is supported out of the box.

**No slashing / no timeouts (v1)**
This version has no penalty for missing contributions. Roadmap v2.0 adds configurable timeout + slashing mechanics.

## State Machine

```
Active (cycle=0, no members)
  → Active (cycle=0, some members joined)
    → Active (cycle=1, group full, running)
      → Active (cycle=N, intermediate cycles)
        → Complete (all payouts done)
```

Transitions:
- `join_group` fills the group → triggers `cycle=1`
- `contribute` (all in) → triggers payout → increments cycle
- Last payout → `status = Complete`
