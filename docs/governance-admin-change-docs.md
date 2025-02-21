# Astroport Governance Admin Change Documentation

## Introduction
This contract enables **governance-driven admin changes** for Astroport contracts.

## Features
- **On-chain governance proposals for admin updates.**
- **Secure voting mechanisms before execution.**
- **Prevents unauthorized modifications.**

## Setup Guide
### Deploy Governance Contract
1. Install dependencies:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
2. Build contract:
   ```bash
   cargo build --release
   ```
3. Deploy:
   ```bash
   terrad tx wasm store path/to/contract.wasm --from wallet --gas auto
   ```

### Submitting an Admin Change Proposal
```bash
terrad tx wasm execute <governance_contract> '{"propose": {"title": "Change Admin", "description": "Updating admin for contract", "proposal_type": "ChangeAdmin", "target_contract": "<astroport_contract>", "new_admin": "terra1newadmin"}}' --from wallet --gas auto
```

### Voting
```bash
terrad tx wasm execute <governance_contract> '{"vote": {"proposal_id": 1, "vote": "yes"}}' --from wallet --gas auto
```

### Executing the Proposal
```bash
terrad tx wasm execute <governance_contract> '{"execute_proposal": {"proposal_id": 1}}' --from wallet --gas auto
```

## Security Best Practices
- Ensure proposals are **reviewed by the community**.
- Use **trusted wallets** to submit execution transactions.

## Community & Support
Join the Terra Classic community for updates.
