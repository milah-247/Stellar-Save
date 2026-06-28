# Vulnerability Response Procedures
<!-- Closes #1175 -->

## Triage Workflow

```
Report received
    │
    ▼
Auto-ack (Day 0)
    │
    ▼
Assign triage lead → classify severity (Day 1)
    │
    ├─ Invalid / Out-of-scope → close with explanation
    │
    └─ Valid
           │
           ▼
       Assign developer + severity label
           │
           ▼
       Develop & review patch (Day 7)
           │
           ▼
       Testnet deploy + reporter verification (Day 14)
           │
           ▼
       Mainnet deploy + reward payment (Day 21)
           │
           ▼
       Public disclosure + CVE filing (Day 28)
```

## Severity Classification

Use CVSS 3.1 base score as a starting point, then adjust for Stellar-Save context:

| CVSS Range | Our Severity | Key Factor |
|------------|-------------|-----------|
| 9.0 – 10.0 | Critical | Smart contract fund loss |
| 7.0 – 8.9  | High     | Privilege escalation, ZK bypass |
| 4.0 – 6.9  | Medium   | Integrity / availability with financial impact |
| 0.1 – 3.9  | Low      | Minor information disclosure |

## Smart Contract Incidents (Critical)

1. **Immediately** pause affected groups via `pause_group()` if the contract supports it
2. Notify Stellar Foundation incident channel
3. Preserve on-chain state for forensic analysis — do **not** clear storage
4. Engage external audit firm for impact assessment before re-deployment

## Communication Templates

### Acknowledgement (Day 0)
> Thank you for your report. We have received it and will triage within 24 hours.
> Reference ID: `BB-YYYYMMDD-NNN`

### Classification Response (Day 1)
> We have classified your finding as **[Severity]**. Expected resolution: Day [N].
> We will contact you before any public disclosure.

### Reward Notification (Day 21)
> Your fix has been deployed. A reward of **$[amount] USDC** will be sent to
> `[Stellar address provided by reporter]` within 24 hours.
