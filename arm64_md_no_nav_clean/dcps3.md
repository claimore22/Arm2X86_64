## DCPS3

Debug change PE state to EL3 This instruction, when executed in Debug state: - If executed at EL3, selects SP_EL3. - Otherwise, changes the current Exception level and SP to EL3 using SP_EL3. The target Exception level of a DCPS3 instruction is EL3. On executing a DCPS3 instruction: - ELR_EL3 becomes UNKNOWN. - SPSR_EL3 becomes UNKNOWN. - ESR_EL3 becomes UNKNOWN. - DLR_EL0 and DSPSR_EL0 become UNKNOWN. - The endianness is set according to SCTLR_EL3.EE. This instruction is always UNDEFINED in Non-debug state. This instruction is UNDEFINED at all Exception levels if either: - EDSCR.SDD == 1. - EL3 is not implemented. For more information on the operation of the DCPS<n> instructions, see DCPS.

#### Decode

// Empty.

### Operation

if \![Halted](shared_pseudocode.html#func_Halted_0)() then Undefined(); end; [DCPSInstruction](shared_pseudocode.html#func_DCPSInstruction_1)([EL3](shared_pseudocode.html#global_EL3));
