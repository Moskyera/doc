package action

import (
	"fmt"

	"github.com/hacash/HVM/eval"
	"github.com/hacash/HVM/trait"
)

// StakeHacd locks one or more HACD literals. VMKind 0x01 (HIP-25).
type StakeHacd struct {
	Diamonds string
}

func (a StakeHacd) VMKind() uint8 { return 0x01 }

func (a StakeHacd) IsBurning90PersentTxFees() bool { return true }

func (a StakeHacd) ChildActions() []trait.VMAction { return nil }

func (a StakeHacd) Parse(_ trait.ExtendCallExecutor, _ []byte, _ uint32) (uint32, error) {
	return 0, fmt.Errorf("StakeHacd.Parse not implemented in scaffold")
}

func (a StakeHacd) Evaluate(ctx trait.Context) trait.EvalResult {
	exec, ok := ctx.GetExtendCallExecutor().(trait.StakingExecutor)
	if !ok {
		return eval.ResultFatalErr(fmt.Errorf("staking executor not registered"))
	}
	err := exec.Stake(ctx.Sender(), a.Diamonds)
	if err != nil {
		return eval.ResultFatalErr(err)
	}
	return eval.ResultValue(nil)
}