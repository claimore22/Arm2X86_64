## DRPS

Debug restore PE state This instruction restores PSTATE from the SPSR. The SPSR is checked for the current Exception level for an illegal return event. See Illegal exception returns from AArch64 state. This instruction is UNDEFINED in Non-debug state. This instruction is UNDEFINED at EL0. For more information on the operation of DRPS, see DRPS.

#### Decode

// Empty.

### Operation

if \![Halted](shared_pseudocode.html#func_Halted_0)() \|\| [PSTATE](shared_pseudocode.html#global_PSTATE).EL == [EL0](shared_pseudocode.html#global_EL0) then Undefined(); end; [DRPSInstruction](shared_pseudocode.html#func_DRPSInstruction_0)();
