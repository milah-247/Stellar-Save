use soroban_sdk::{contracttype, Address, Vec};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum GroupStatus {
    Active,
    Complete,
}

/// Core ROSCA group state stored on-chain.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Group {
    /// Amount each member must contribute per cycle (in stroops).
    pub contribution_amount: i128,
    /// Duration of each cycle in ledgers.
    pub cycle_duration: u32,
    /// Maximum number of members allowed.
    pub max_members: u32,
    /// Current members in join order.
    pub members: Vec<Address>,
    /// Index of the next member to receive payout (0-based).
    pub payout_index: u32,
    /// Current cycle number (1-based, 0 = not started).
    pub current_cycle: u32,
    /// Ledger number when the current cycle started.
    pub cycle_start_ledger: u32,
    pub status: GroupStatus,
}

/// Persistent storage keys.
#[contracttype]
pub enum DataKey {
    /// Counter for the next group ID.
    GroupCounter,
    /// Group state by ID.
    Group(u64),
    /// Whether a member has contributed in a given cycle: (group_id, cycle, member).
    Contributed(u64, u32, Address),
}
