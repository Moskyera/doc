//! HIP-25: Pure HACD Staking actions for hacash/rust
//! Target: src/mint/action/diamond_staking.rs

/// 22% of eligible inscription + transfer fees
pub const STAKING_FEE_SHARE_PERCENT: u64 = 22;

/// ~3 days cooldown after unstake request
pub const COOLDOWN_BLOCKS: u64 = 864;

/// ~3 months minimum stake before unstake allowed
pub const MIN_STAKE_BLOCKS: u64 = 25714;

pub const DIAMOND_STATUS_STAKED: Uint1 = Uint1::from(4);
pub const DIAMOND_STATUS_STAKING_COOLDOWN: Uint1 = Uint1::from(5);

/**
 * Diamond Stake — lock HACD literals and start earning
 * Action kind ID: propose 34 (after DiamondInscriptionClear 33)
 */
ActionDefine!{
    DiamondStake : 34, (
        diamonds : DiamondNameListMax200
    ),
    ACTLV_MAIN,
    21,
    (self, ctx, state, store, gas),
    true, // burn 90%
    [],
    {
        gas += self.diamonds.count().uint() as i64 * DiamondName::width() as i64;
        let staker = ctx.main_address().clone();
        let height = ctx.pending_height();
        do_diamonds_stake(&self.diamonds, staker, height, ctx, state, store)
    }
}

/**
 * Diamond Unstake — begin cooldown; rewards paid at unlock
 * Action kind ID: propose 35
 */
ActionDefine!{
    DiamondUnstake : 35, (
        diamonds : DiamondNameListMax200
    ),
    ACTLV_MAIN,
    21,
    (self, ctx, state, store, gas),
    true,
    [],
    {
        gas += self.diamonds.count().uint() as i64 * DiamondName::width() as i64;
        let staker = ctx.main_address().clone();
        let height = ctx.pending_height();
        do_diamonds_unstake(&self.diamonds, staker, height, ctx, state, store)
    }
}

fn do_diamonds_stake(
    diamonds: &DiamondNameListMax200,
    staker: Address,
    height: u64,
    ctx: &mut dyn ExecContext,
    sta: &mut dyn State,
    _sto: &dyn Store,
) -> Ret<Vec<u8>> {
    diamonds.check()?;
    let mut state = MintState::wrap(sta);
    ensure_staking_not_paused(&state)?;

    let global_index = staking_global_reward_index(&state);

    for dianame in diamonds.list() {
        let mut diaitem = check_diamond_status(&state, &staker, &dianame)?;
        if diaitem.status != DIAMOND_STATUS_NORMAL {
            return errf!(
                "diamond {} cannot be staked (status {})",
                dianame.readable(),
                diaitem.status.uint()
            );
        }
        diaitem.status = DIAMOND_STATUS_STAKED;
        staking_attach_record(&mut diaitem, &staker, height, global_index);
        state.set_diamond(&dianame, &diaitem);
        staking_increment_shares(&mut state, 1)?;
        emit_staked(&dianame, &staker, height);
    }

    drop(state);
    Ok(vec![])
}

fn do_diamonds_unstake(
    diamonds: &DiamondNameListMax200,
    staker: Address,
    height: u64,
    ctx: &mut dyn ExecContext,
    sta: &mut dyn State,
    _sto: &dyn Store,
) -> Ret<Vec<u8>> {
    diamonds.check()?;
    let mut state = MintState::wrap(sta);

    for dianame in diamonds.list() {
        let mut diaitem = must_have!(
            format!("diamond {}", dianame.readable()),
            state.diamond(&dianame)
        );
        staking_ensure_staker(&diaitem, &staker, &dianame)?;
        if diaitem.status != DIAMOND_STATUS_STAKED {
            return errf!("diamond {} is not staked", dianame.readable());
        }
        let stake_height = staking_stake_height(&diaitem)?;
        if height < stake_height + MIN_STAKE_BLOCKS {
            return errf!(
                "diamond {} must be staked for at least {} blocks (~3 months)",
                dianame.readable(),
                MIN_STAKE_BLOCKS
            );
        }

        let global_index = staking_global_reward_index(&state);
        let reward = staking_accrued_reward(&diaitem, global_index);

        diaitem.status = DIAMOND_STATUS_STAKING_COOLDOWN;
        staking_set_cooldown(&mut diaitem, height + COOLDOWN_BLOCKS, global_index);
        state.set_diamond(&dianame, &diaitem);
        staking_decrement_shares(&mut state, 1)?;
        staking_hold_pending_reward(&mut state, &dianame, reward, height + COOLDOWN_BLOCKS)?;

        emit_unstake_requested(&dianame, &staker, height, height + COOLDOWN_BLOCKS, reward);
    }

    drop(state);
    Ok(vec![])
}

/// Redirect eligible fee: 22% to pool, remainder to burn path
pub fn redirect_staking_fee(total_fee: Amount, state: &mut MintState) -> (Amount, Amount) {
    let total = total_fee.uint() as u64;
    let to_pool = total * STAKING_FEE_SHARE_PERCENT / 100;
    let to_burn = total - to_pool;
    staking_pool_deposit(state, to_pool);
    (Amount::from(to_pool), Amount::from(to_burn))
}

// --- Staking state helpers (implement in mint/operate/staking.rs) ---

fn ensure_staking_not_paused(_state: &MintState) -> Ret<()> {
    // read GlobalStakingState.paused
    Ok(())
}

fn staking_global_reward_index(_state: &MintState) -> u128 {
    0
}

fn staking_increment_shares(_state: &mut MintState, _n: u64) -> Ret<()> {
    Ok(())
}

fn staking_decrement_shares(_state: &mut MintState, _n: u64) -> Ret<()> {
    Ok(())
}

fn staking_attach_record(_dia: &mut DiamondSto, _staker: &Address, _height: u64, _index: u128) {
    // set stake_height, reward_index snapshot, staker address in extension fields
}

fn staking_ensure_staker(_dia: &DiamondSto, _staker: &Address, _name: &DiamondName) -> Ret<()> {
    Ok(())
}

fn staking_stake_height(_dia: &DiamondSto) -> Ret<u64> {
    Ok(0)
}

fn staking_accrued_reward(_dia: &DiamondSto, _global_index: u128) -> u128 {
    0
}

fn staking_set_cooldown(_dia: &mut DiamondSto, _unlock_height: u64, _index: u128) {}

fn staking_hold_pending_reward(
    _state: &mut MintState,
    _name: &DiamondName,
    _reward: u128,
    _unlock_height: u64,
) -> Ret<()> {
    Ok(())
}

fn staking_pool_deposit(_state: &mut MintState, _amount: u64) {}

fn emit_staked(_name: &DiamondName, _staker: &Address, _height: u64) {}
fn emit_unstake_requested(
    _name: &DiamondName,
    _staker: &Address,
    _height: u64,
    _unlock: u64,
    _reward: u128,
) {
}