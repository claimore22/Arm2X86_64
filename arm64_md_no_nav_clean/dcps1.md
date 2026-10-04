## DCPS1

Debug change PE state to EL1 This instruction, when executed in Debug state: - If executed at EL0, changes the current Exception level and SP to EL1 using SP_EL1. - Otherwise, if executed at ELx, selects SP_ELx. The target Exception level of a DCPS1 instruction is: - EL1 if the instruction is executed at EL0. - Otherwise, the Exception level at which the instruction is executed. When the target Exception level of a DCPS1 instruction is ELx, on executing this instruction: - ELR_ELx becomes UNKNOWN. - SPSR_ELx becomes UNKNOWN. - ESR_ELx becomes UNKNOWN. - DLR_EL0 and DSPSR_EL0 become UNKNOWN. - The endianness is set according to SCTLR_ELx.EE. This instruction is always UNDEFINED in Non-debug state. This instruction is UNDEFINED at EL0 if EL2 is implemented and enabled in the current Security state and HCR_EL2.TGE == 1. For more information on the operation of the DCPS<n> instructions, see DCPS.

#### Decode

// Empty.

### Operation

if \![Halted](shared_pseudocode.html#func_Halted_0)() then Undefined(); end; [DCPSInstruction](shared_pseudocode.html#func_DCPSInstruction_1)([EL1](shared_pseudocode.html#global_EL1));
