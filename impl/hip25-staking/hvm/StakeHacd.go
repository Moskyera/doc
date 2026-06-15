package action

import (
	"fmt"

	"github.com/hacash/HVM/eval"
	"github.com/hacash/HVM/trait"
)

// StakeHacd locks one or more HACD literals. VMKind 0x01 (HIP-25).
type StakeHacd struct {
	Diamonds []byte
}

func (a *StakeHacd) VMKind() uint8 { return 0x01 }

func (a *StakeHacd) IsBurning90PersentTxFees() bool { return true }

func (a *StakeHacd) ChildActions() []trait.VMAction { return nil }

func (a *StakeHacd) Parse(_ trait.ExtendCallExecutor, buf []byte, seek uint32) (uint32, error) {
	names, next, err := parseDiamondNameList(buf, seek+1)
	if err != nil {
		return 0, err
	}
	a.Diamonds = names
	return next, nil
}

func (a *StakeHacd) Evaluate(ctx trait.Context) trait.EvalResult {
	handler, ok := ctx.GetExtendCallExecutor().(trait.StakingHandler)
	if !ok {
		return eval.ResultFatalErr(fmt.Errorf("staking handler not registered"))
	}
	tcx, ok := ctx.(trait.TransactionContext)
	if !ok {
		return eval.ResultFatalErr(fmt.Errorf("transaction context required for staking"))
	}
	if err := handler.Stake(tcx.Sender(), a.Diamonds); err != nil {
		return eval.ResultFatalErr(err)
	}
	return eval.ResultValue(nil)
}