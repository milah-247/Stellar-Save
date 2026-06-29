# Roadmap

## v1.0 — Current

- [x] ROSCA core: create group, join, contribute, payout rotation
- [x] Native XLM support via SEP-41 token interface
- [x] Automatic payout when all members contribute
- [x] Manual `execute_payout` fallback
- [x] Comprehensive test suite (17 tests)
- [x] Testnet deploy script

## v1.1 — Token Support

- [ ] Store `token` address inside `Group` at creation time
- [ ] Validate token address on every contribution and payout
- [ ] Support USDC, EURC, and any SEP-41 token
- [ ] Emit contract events for contributions and payouts

## v2.0 — Resilience & Incentives

- [ ] Configurable `timeout_ledger` per group
- [ ] `slash_member()` — eject non-contributing members after timeout, refund others
- [ ] Randomized payout order (opt-in at group creation)
- [ ] Penalty bonds: members lock a small deposit at join, forfeited on slash
- [ ] Auto-extend storage TTL on every write

## v3.0 — Frontend

- [ ] React + Freighter wallet integration
- [ ] Group discovery UI (browse open groups)
- [ ] Contribution status dashboard
- [ ] Push notifications for cycle starts
- [ ] [Design token system & dark mode](design-tokens.md) (#1173)
- [ ] [Funnel & cohort analytics](funnel-analytics.md) (#1172)

## v4.0 — Mobile & Fiat

- [ ] React Native mobile app
- [ ] Fiat on/off-ramp integration (MoneyGram, mobile money)
- [ ] Localization: Yoruba, Igbo, Hausa, Swahili, French
- [ ] QR-code invite links for group joining

## Long-term Research

- [ZK-based private contribution amounts](zk-verification.md) — prove attributes without on-chain disclosure (#1174)
- Cross-chain ROSCA groups (Stellar ↔ EVM via Axelar)
- DAO governance for contract upgrades

## Security

- [Bug Bounty & Vulnerability Disclosure](../security/BUG_BOUNTY.md) (#1175)
