# Disciplr Soroban Contracts

On-chain programmable, time-locked capital vaults for accountability staking,
the chain-side counterpart to the `disciplr-backend` API and Horizon listener.

## Workspace layout

```text
contracts/
├── Cargo.toml                       # workspace manifest (soroban-sdk = "23")
├── README.md
└── accountability_vault/
    ├── Cargo.toml
    └── src/
        ├── lib.rs                   # AccountabilityVault contract
        └── test.rs                  # unit tests (testutils)
```

## accountability_vault

Implements the vault lifecycle that the backend models off-chain in
`src/services/vaultTransitions.ts` and parses events for in
`src/services/eventParser.ts`:

| Function | Purpose |
|---|---|
| `create_vault` | Create a `Draft` vault with milestones, verifier, and success/failure destinations. Validates amount, deadline, and that milestone amounts sum to the total. |
| `stake` | Creator transfers the SEP-41 token into the contract; `Draft` -> `Active`. |
| `check_in` | Designated verifier confirms a milestone before its `due_date`. |
| `slash_on_miss` | After the deadline with unverified milestones, slash funds to `failure_destination`; `Active` -> `Failed`. |
| `claim` | When all milestones are verified, release funds to `success_destination`; `Active` -> `Completed`. |
| `withdraw` | Cancel/refund an unfunded or unstarted vault to the creator; -> `Cancelled`. |
| `get_vault` | Read-only accessor for the current vault record. |
| `get_status` | Read-only accessor for the current `VaultStatus`, useful for cheap off-chain reconciliation. |
| `get_milestone_status` | Read-only accessor for a milestone's `verified` flag and `due_date`; rejects out-of-range indexes. |

The `VaultStatus` enum (`Draft`/`Active`/`Completed`/`Failed`/`Cancelled`)
mirrors `PersistedVault.status` in `src/types/vaults.ts`. Emitted events
(`vault_created`, `vault_staked`, `milestone_checked_in`, `vault_slashed`,
`vault_completed`, `vault_cancelled`, `vault_withdrawn`) align with the topics
consumed by the backend event parser.

`get_status` and `get_milestone_status(index)` do not require authentication
and do not mutate storage. They return compact Soroban contract types with
stable field names so backend callers can decode them with `scValToNative`
without reconstructing the entire vault from events.

## Build & test

```bash
# from the contracts/ directory
stellar contract build
cargo test
```

## Backend integration

`src/services/soroban.ts` calls `create_vault` via the Stellar SDK
(`@stellar/stellar-sdk` v14). The Horizon listener
(`src/services/horizonListener.ts`) and `src/services/eventParser.ts`
ingest the events emitted by these functions to keep the off-chain vault state
in sync.
