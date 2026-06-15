package action

import (
	"fmt"

	"github.com/hacash/HVM/eval"
	"github.com/hacash/HVM/trait"
)

// UnstakeHacd begins the 864-block cooldown. VMKind 0x02 (HIP-25).
// Requires MIN_STAKE_BLOCKS (25714) elapsed since stake.
type UnstakeHacd struct {
	Diamonds string
}

func (a UnstakeHacd) VMKind() uint8 { return 0x02 }

func (a UnstakeHacd) IsBurning90PersentTxFees() bool { return true }

func (a UnstakeHacd) ChildActions() []trait.VMAction { return nil }

func (a UnstakeHacd) Parse(_ trait.ExtendCallExecutor, _ []byte, _ uint32) (uint32, error) {
	return 0, fmt.Errorf("UnstakeHacd.Parse not implemented in scaffold")
}

func (a UnstakeHacd) Evaluate(ctx trait.Context) trait.EvalResult {
	exec, ok := ctx.GetExtendCallExecutor().(trait.StakingExecutor)
	if !ok {
		return eval.ResultFatalErr(fmt.Errorf("staking executor not registered"))
	}
	err := exec.Unstake(ctx.Sender(), a.Diamonds)
	if err != nil {
		return eval.ResultFatalErr(err)
	}
	return eval.ResultValue(nil)
}