package action

import (
	"fmt"

	"github.com/hacash/HVM/eval"
	"github.com/hacash/HVM/trait"
)

// UnstakeHacd begins the 864-block cooldown. VMKind 0x02 (HIP-25).
// Requires MIN_STAKE_BLOCKS (25714) elapsed since stake (enforced in Mint layer).
type UnstakeHacd struct {
	Diamonds []byte
}

func (a *UnstakeHacd) VMKind() uint8 { return 0x02 }

func (a *UnstakeHacd) IsBurning90PersentTxFees() bool { return true }

func (a *UnstakeHacd) ChildActions() []trait.VMAction { return nil }

func (a *UnstakeHacd) Parse(_ trait.ExtendCallExecutor, buf []byte, seek uint32) (uint32, error) {
	names, next, err := parseDiamondNameList(buf, seek+1)
	if err != nil {
		return 0, err
	}
	a.Diamonds = names
	return next, nil
}

func (a *UnstakeHacd) Evaluate(ctx trait.Context) trait.EvalResult {
	handler, ok := ctx.GetExtendCallExecutor().(trait.StakingHandler)
	if !ok {
		return eval.ResultFatalErr(fmt.Errorf("staking handler not registered"))
	}
	tcx, ok := ctx.(trait.TransactionContext)
	if !ok {
		return eval.ResultFatalErr(fmt.Errorf("transaction context required for staking"))
	}
	if err := handler.Unstake(tcx.Sender(), a.Diamonds); err != nil {
		return eval.ResultFatalErr(err)
	}
	return eval.ResultValue(nil)
}