# Bug Bounty Program
<!-- Closes #1175 -->

Stellar-Save runs a responsible disclosure programme to incentivise security researchers to report vulnerabilities safely.

## Scope

**In scope**
- Soroban smart contracts (`contracts/`)
- ZK circuits and proof verification (`zk/`)
- Backend API services
- Frontend wallet integration

**Out of scope**
- Third-party infrastructure (Stellar network, HackerOne platform)
- Social-engineering attacks against team members
- Denial-of-service without demonstrable asset loss
- Issues already known or publicly disclosed

---

## Vulnerability Classifications & Rewards

| Severity | Description | Reward (USD) |
|----------|-------------|-------------|
| **Critical** | Direct fund loss, unauthorized payout execution, smart contract drain | $5,000 – $20,000 |
| **High** | Privilege escalation, bypass of contribution validation, ZK proof forgery | $1,000 – $5,000 |
| **Medium** | Data integrity issues, denial-of-service with financial impact, incorrect state transitions | $250 – $1,000 |
| **Low** | Information disclosure, minor logic bugs, UI/UX security issues | $50 – $250 |

Rewards are paid in USDC on Stellar after fix verification.

---

## How to Report

1. **Email** `security@stellar-save.example` with subject `[BUG BOUNTY] <short title>`
2. **Encrypt** the report using our PGP key (see `SECURITY.md`)
3. Include:
   - Description and impact
   - Step-by-step reproduction
   - Affected component and version
   - Suggested fix (optional)

Alternatively, submit through our **HackerOne programme** at `https://hackerone.com/stellar-save` *(to be activated on programme launch)*.

---

## Response Timeline

| Day | Action |
|-----|--------|
| **0** | Report received — automated acknowledgement sent |
| **1** | Triage: severity classification communicated to reporter |
| **7** | Patch developed and internally reviewed |
| **14** | Fix deployed to testnet; reporter invited to verify |
| **21** | Fix deployed to mainnet; reward issued |
| **28** | Coordinated public disclosure (CVE filed if applicable) |

We may extend the timeline for Critical findings requiring trusted-setup re-ceremonies or third-party audits; reporters are notified promptly.

---

## Researcher Responsibilities

- Do **not** exploit a vulnerability beyond what is needed to demonstrate impact
- Do **not** access, modify, or exfiltrate user data
- Do **not** perform testing on mainnet without prior written approval
- Keep findings confidential until the agreed disclosure date

---

## Legal Safe Harbour

Researchers acting in good faith in accordance with this policy will not face legal action from Stellar-Save. We consider responsible security research a public good.

---

## HackerOne Setup Checklist

- [ ] Create HackerOne programme (private → public after initial review)
- [ ] Configure scope and out-of-scope assets
- [ ] Set reward ranges per severity tier (see table above)
- [ ] Add PGP public key to `SECURITY.md`
- [ ] Assign triage team with on-call rotation
- [ ] Link programme URL from `README.md` and `SECURITY.md`
