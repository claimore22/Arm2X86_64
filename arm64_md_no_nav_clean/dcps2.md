## DCPS2

Debug change PE state to EL2 This instruction, when executed in Debug state: - If executed at EL0 or EL1, changes the current Exception level and SP to EL2 using SP_EL2. - Otherwise, if executed at ELx, selects SP_ELx. The target Exception level of a DCPS2 instruction is: - EL2 if the instruction is executed at an Exception level that is not EL3. - EL3 if the instruction is executed at EL3. When the target Exception level of a DCPS2 instruction is ELx, on executing this instruction: - ELR_ELx becomes UNKNOWN. - SPSR_ELx becomes UNKNOWN. - ESR_ELx becomes UNKNOWN. - DLR_EL0 and DSPSR_EL0 become UNKNOWN. - The endianness is set according to SCTLR_ELx.EE. This instruction is always UNDEFINED in Non-debug state. This instruction is UNDEFINED at the following Exception levels: - All Exception levels if EL2 is not implemented. - At EL0 and EL1 if EL2 is disabled in the current Security state. For more information on the operation of the DCPS<n> instructions, see DCPS.

#### Decode

// Empty.

### Operation

if \![Halted](shared_pseudocode.html#func_Halted_0)() then Undefined(); end; [DCPSInstruction](shared_pseudocode.html#func_DCPSInstruction_1)([EL2](shared_pseudocode.html#global_EL2));
