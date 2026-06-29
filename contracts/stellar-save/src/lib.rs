#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, Vec};

mod error;
mod types;
mod xlm;

use error::Error;
use types::{DataKey, Group, GroupStatus};

#[contract]
pub struct StellarSave;

#[contractimpl]
impl StellarSave {
    // ─── Group Management ───────────────────────────────────────────────────

    /// Create a new ROSCA group. Returns the new group ID.
    ///
    /// * `contribution_amount` – stroops each member contributes per cycle.
    /// * `cycle_duration`      – cycle length in ledgers.
    /// * `max_members`         – 2 ≤ max_members ≤ 20.
    pub fn create_group(
        env: Env,
        contribution_amount: i128,
        cycle_duration: u32,
        max_members: u32,
    ) -> Result<u64, Error> {
        if contribution_amount <= 0 || cycle_duration == 0 || max_members < 2 || max_members > 20 {
            return Err(Error::InvalidConfig);
        }

        let id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::GroupCounter)
            .unwrap_or(0u64);

        let group = Group {
            contribution_amount,
            cycle_duration,
            max_members,
            members: Vec::new(&env),
            payout_index: 0,
            current_cycle: 0,
            cycle_start_ledger: 0,
            status: GroupStatus::Active,
        };

        env.storage().persistent().set(&DataKey::Group(id), &group);
        env.storage()
            .instance()
            .set(&DataKey::GroupCounter, &(id + 1));

        Ok(id)
    }

    /// Get group state by ID.
    pub fn get_group(env: Env, group_id: u64) -> Result<Group, Error> {
        Self::load_group(&env, group_id)
    }

    /// List all member addresses in a group.
    pub fn list_members(env: Env, group_id: u64) -> Result<Vec<Address>, Error> {
        Ok(Self::load_group(&env, group_id)?.members)
    }

    // ─── Membership ──────────────────────────────────────────────────────────

    /// Join an existing group. Requires auth from `member`.
    pub fn join_group(env: Env, group_id: u64, member: Address) -> Result<(), Error> {
        member.require_auth();

        let mut group = Self::load_group(&env, group_id)?;

        if group.status == GroupStatus::Complete {
            return Err(Error::GroupComplete);
        }
        if group.members.len() >= group.max_members {
            return Err(Error::GroupFull);
        }
        if group.members.contains(&member) {
            return Err(Error::AlreadyMember);
        }

        group.members.push_back(member.clone());

        // Start the first cycle once the group is full.
        if group.members.len() == group.max_members {
            group.current_cycle = 1;
            group.cycle_start_ledger = env.ledger().sequence();
        }

        env.storage().persistent().set(&DataKey::Group(group_id), &group);
        Ok(())
    }

    /// Check if an address is a member of the group.
    pub fn is_member(env: Env, group_id: u64, address: Address) -> Result<bool, Error> {
        Ok(Self::load_group(&env, group_id)?.members.contains(&address))
    }

    // ─── Contributions ───────────────────────────────────────────────────────

    /// Contribute for the current cycle. Transfers `contribution_amount` from
    /// `member` to the contract. Automatically triggers payout when all members
    /// have contributed.
    pub fn contribute(
        env: Env,
        group_id: u64,
        member: Address,
        token: Address,
    ) -> Result<(), Error> {
        member.require_auth();

        let group = Self::load_group(&env, group_id)?;

        if group.status == GroupStatus::Complete {
            return Err(Error::GroupComplete);
        }
        if group.current_cycle == 0 {
            // Group not full yet
            return Err(Error::CyclePending);
        }
        if !group.members.contains(&member) {
            return Err(Error::NotMember);
        }

        let cycle = group.current_cycle;
        let contrib_key = DataKey::Contributed(group_id, cycle, member.clone());

        if env.storage().persistent().has(&contrib_key) {
            return Err(Error::AlreadyContributed);
        }

        // Transfer contribution into the contract.
        xlm::transfer(
            &env,
            &token,
            &member,
            &env.current_contract_address(),
            group.contribution_amount,
        );

        env.storage().persistent().set(&contrib_key, &true);

        // Check if all members have now contributed this cycle.
        let all_contributed = group.members.iter().all(|m| {
            env.storage()
                .persistent()
                .has(&DataKey::Contributed(group_id, cycle, m))
        });

        if all_contributed {
            Self::do_payout(&env, group_id, token)?;
        }

        Ok(())
    }

    /// Return contribution status for every member in `cycle_number`.
    /// Returns a parallel vec of booleans (true = contributed).
    pub fn get_contribution_status(
        env: Env,
        group_id: u64,
        cycle_number: u32,
    ) -> Result<Vec<bool>, Error> {
        let group = Self::load_group(&env, group_id)?;
        let mut statuses = Vec::new(&env);
        for m in group.members.iter() {
            let contributed = env
                .storage()
                .persistent()
                .has(&DataKey::Contributed(group_id, cycle_number, m));
            statuses.push_back(contributed);
        }
        Ok(statuses)
    }

    // ─── Payouts ─────────────────────────────────────────────────────────────

    /// Manually trigger a payout if all contributions for the current cycle
    /// are in. Normally called automatically by `contribute`.
    pub fn execute_payout(env: Env, group_id: u64, token: Address) -> Result<(), Error> {
        let group = Self::load_group(&env, group_id)?;
        if group.status == GroupStatus::Complete {
            return Err(Error::GroupComplete);
        }
        if group.current_cycle == 0 {
            return Err(Error::CyclePending);
        }

        let cycle = group.current_cycle;
        let all_contributed = group.members.iter().all(|m| {
            env.storage()
                .persistent()
                .has(&DataKey::Contributed(group_id, cycle, m))
        });

        if !all_contributed {
            return Err(Error::CyclePending);
        }

        Self::do_payout(&env, group_id, token)
    }

    /// Whether the ROSCA has completed all cycles.
    pub fn is_complete(env: Env, group_id: u64) -> Result<bool, Error> {
        Ok(Self::load_group(&env, group_id)?.status == GroupStatus::Complete)
    }

    // ─── Internal helpers ────────────────────────────────────────────────────

    fn load_group(env: &Env, group_id: u64) -> Result<Group, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Group(group_id))
            .ok_or(Error::GroupNotFound)
    }

    /// Pay the current recipient and advance the cycle (or mark complete).
    fn do_payout(env: &Env, group_id: u64, token: Address) -> Result<(), Error> {
        let mut group = Self::load_group(env, group_id)?;

        let recipient = group
            .members
            .get(group.payout_index)
            .ok_or(Error::GroupNotFound)?;

        let payout_amount = group.contribution_amount * group.members.len() as i128;

        xlm::transfer(
            env,
            &token,
            &env.current_contract_address(),
            &recipient,
            payout_amount,
        );

        group.payout_index += 1;

        if group.payout_index >= group.members.len() {
            // All members have received their payout.
            group.status = GroupStatus::Complete;
        } else {
            group.current_cycle += 1;
            group.cycle_start_ledger = env.ledger().sequence();
        }

        env.storage().persistent().set(&DataKey::Group(group_id), &group);
        Ok(())
    }
}

mod test;
