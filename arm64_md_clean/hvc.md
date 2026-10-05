## HVC

Hypervisor call This instruction causes an exception to EL2. Software executing at EL1 can use this instruction to call the hypervisor to request a service. The HVC instruction is UNDEFINED: - When EL3 is implemented and SCR_EL3.HCE is set to 0. - When EL3 is not implemented and HCR_EL2.HCD is set to 1. - When EL2 is not implemented. - At EL1 if EL2 is not enabled in the current Security state. - At EL0. On executing an HVC instruction, the PE records the exception as a Hypervisor Call exception in ESR_ELx, using the EC syndrome value 0x16, and the value of the immediate argument.

#### Decode

if \![HaveEL](shared_pseudocode.html#func_HaveEL_1)([EL2](shared_pseudocode.html#global_EL2)) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let imm : bits(16) = imm16;

### Operation

if [PSTATE](shared_pseudocode.html#global_PSTATE).EL == [EL0](shared_pseudocode.html#global_EL0) then Undefined(); end; if [PSTATE](shared_pseudocode.html#global_PSTATE).EL == [EL1](shared_pseudocode.html#global_EL1) && \![EL2Enabled](shared_pseudocode.html#func_EL2Enabled_0)() then Undefined(); end; if \![HaveEL](shared_pseudocode.html#func_HaveEL_1)([EL3](shared_pseudocode.html#global_EL3)) && HCR_EL2().HCD == '1' then Undefined(); end; if [HaveEL](shared_pseudocode.html#func_HaveEL_1)([EL3](shared_pseudocode.html#global_EL3)) && SCR_EL3().HCE == '0' then Undefined(); end; [AArch64_CallHypervisor](shared_pseudocode.html#func_AArch64_CallHypervisor_1)(imm);
