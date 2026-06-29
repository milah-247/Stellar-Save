# Threat Model & Security

## Trust Assumptions

- **No admin/owner**: The contract has no privileged account. Once deployed, all rules are enforced deterministically by code.
- **Stellar consensus**: All state transitions rely on Stellar's BFT consensus; reorgs are not a concern.
- **Token contract**: The contract trusts the SEP-41 token passed as `token` at call time. Callers should verify they are passing the correct token address.

## Identified Risks & Mitigations

### 1. Member refuses to contribute (griefing)

**Risk**: A member joins but never calls `contribute()`, blocking the cycle indefinitely.

**Current mitigation (v1)**: None. All members must contribute before a payout fires.

**Roadmap mitigation (v2)**: Configurable `timeout_ledger`. If a member hasn't contributed after `cycle_start_ledger + timeout_ledger`, anyone can call `slash_member()` to eject the member and refund the other contributors.

---

### 2. Front-running payout order

**Risk**: A malicious member joins last to secure a specific payout position.

**Current mitigation**: Payout order is simply join order, which is transparent and predictable. Future versions may allow randomized order (v2 roadmap).

---

### 3. Reentrancy via token callback

**Risk**: A malicious SEP-41 token could reenter the contract during `transfer()`.

**Mitigation**: State updates (writing `Contributed` key, updating `Group`) happen *before* outbound transfers in `contribute()`. The payout transfer in `do_payout()` is the last operation after all state is committed — consistent with checks-effects-interactions.

---

### 4. Integer overflow in payout amount

**Risk**: `contribution_amount × members.len()` could overflow `i128`.

**Mitigation**: `max_members` is capped at 20. Maximum practical contribution is bounded by XLM supply (~50B XLM = 5×10¹⁶ stroops). `20 × 5×10¹⁶ = 10¹⁸`, well within `i128::MAX` (~1.7×10³⁸).

---

### 5. Storage TTL expiry

**Risk**: Soroban persistent storage entries expire if not extended. A group's state could be archived before all cycles complete.

**Mitigation**: Operators should monitor TTLs and extend them proactively. A future version will call `extend_ttl` on every state write.

---

### 6. Wrong token address supplied

**Risk**: A caller passes a different token address than what other members use, redirecting funds.

**Mitigation (planned v1.1)**: Store `token` address inside `Group` at creation time and validate it on every `contribute`/`payout` call. For v1, all callers must agree out-of-band on the token address (this is easy when using native XLM).

## Audit Status

v1.0 has not undergone a third-party audit. **Do not use with significant real funds until audited.**

## Reporting Vulnerabilities

See [SECURITY.md](../SECURITY.md) for the responsible disclosure process.
