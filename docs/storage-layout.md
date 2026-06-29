# Storage Layout

All state is stored in Soroban **persistent** storage (survives ledger archiving) except the group counter which is in **instance** storage.

## Instance Storage

| Key | Type | Description |
|-----|------|-------------|
| `GroupCounter` | `u64` | Monotonically increasing ID for the next group |

## Persistent Storage

### Group State

| Key | Type | Description |
|-----|------|-------------|
| `Group(group_id: u64)` | `Group` | Full group state |

**`Group` struct fields:**

| Field | Type | Description |
|-------|------|-------------|
| `contribution_amount` | `i128` | Per-member contribution in stroops |
| `cycle_duration` | `u32` | Cycle length in ledgers |
| `max_members` | `u32` | Max members (2–20) |
| `members` | `Vec<Address>` | Members in join order; also payout order |
| `payout_index` | `u32` | Index of the next recipient in `members` |
| `current_cycle` | `u32` | 0 = not started, 1+ = active cycle number |
| `cycle_start_ledger` | `u32` | Ledger when current cycle began |
| `status` | `GroupStatus` | `Active` or `Complete` |

### Contribution Tracking

| Key | Type | Description |
|-----|------|-------------|
| `Contributed(group_id, cycle, member)` | `bool` | Whether `member` has contributed in `cycle` of `group_id` |

## Storage Growth

For a group with `N` members running `N` cycles:

- 1 `Group` entry (updated in-place each cycle)
- `N × N` `Contributed` entries created over the lifetime

A 10-member group produces 101 storage entries total. All fit comfortably within Soroban limits.

## Archiving

Persistent entries on Stellar have TTLs. Production deployments should extend TTLs via `extend_ttl` calls on long-running groups. A helper script or off-chain keepalive service is recommended for groups with cycle durations exceeding ~100k ledgers.
