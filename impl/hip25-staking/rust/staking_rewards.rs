//! HIP-25 per-block reward distribution
//! Target: src/mint/operate/staking_rewards.rs

pub fn on_block_close(state: &mut MintState, height: u64) -> Ret<()> {
    distribute_staking_rewards(state)?;
    finalize_staking_cooldowns(state, height)?;
    Ok(())
}

pub fn distribute_staking_rewards(state: &mut MintState) -> Ret<()> {
    let shares = staking_total_shares(state);
    let pool = staking_pool_balance(state);
    if shares == 0 || pool == 0 {
        return Ok(());
    }
    let increment = pool / shares as u128;
    staking_add_global_index(state, increment);
    staking_clear_pool(state);
    emit_reward_distributed(increment, shares);
    Ok(())
}

pub fn finalize_staking_cooldowns(state: &mut MintState, height: u64) -> Ret<()> {
    for (literal, reward) in staking_pending_unlocks_at(state, height) {
        let mut diaitem = must_have!(
            format!("diamond {}", literal.readable()),
            state.diamond(&literal)
        );
        diaitem.status = DIAMOND_STATUS_NORMAL;
        staking_clear_record(&mut diaitem);
        state.set_diamond(&literal, &diaitem);
        staking_pay_reward(state, &diaitem.address, reward)?;
        emit_unstaked(&literal, &diaitem.address, height, reward);
    }
    Ok(())
}

fn staking_total_shares(_state: &MintState) -> u64 {
    0
}
fn staking_pool_balance(_state: &MintState) -> u128 {
    0
}
fn staking_add_global_index(_state: &mut MintState, _inc: u128) {}
fn staking_clear_pool(_state: &mut MintState) {}
fn staking_pending_unlocks_at(_state: &MintState, _height: u64) -> Vec<(DiamondName, u128)> {
    vec![]
}
fn staking_clear_record(_dia: &mut DiamondSto) {}
fn staking_pay_reward(_state: &mut MintState, _addr: &Address, _reward: u128) -> Ret<()> {
    Ok(())
}
fn emit_reward_distributed(_inc: u128, _shares: u64) {}
fn emit_unstaked(_name: &DiamondName, _staker: &Address, _height: u64, _reward: u128) {}