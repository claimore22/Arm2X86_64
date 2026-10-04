## SMC

Secure monitor call This instruction causes an exception to EL3. SMC is available only for software executing at EL1 or higher. It is UNDEFINED at EL0. If the values of HCR_EL2.TSC and SCR_EL3.SMD are both 0, execution of an SMC instruction at EL1 or higher generates a Secure Monitor Call exception, recording it in ESR_ELx, using the EC syndrome value 0x17, that is taken to EL3. If the value of HCR_EL2.TSC is 1 and EL2 is enabled in the current Security state, execution of an SMC instruction at EL1 generates an exception that is taken to EL2, regardless of the value of SCR_EL3.SMD. If the value of HCR_EL2.TSC is 0 and the value of SCR_EL3.SMD is 1, the SMC instruction is UNDEFINED.

#### Decode

let imm : bits(16) = imm16;

### Operation

if [PSTATE](shared_pseudocode.html#global_PSTATE).EL == [EL0](shared_pseudocode.html#global_EL0) then Undefined(); end; [AArch64_CheckForSMCUndefOrTrap](shared_pseudocode.html#func_AArch64_CheckForSMCUndefOrTrap_1)(imm); [AArch64_CallSecureMonitor](shared_pseudocode.html#func_AArch64_CallSecureMonitor_1)(imm);
