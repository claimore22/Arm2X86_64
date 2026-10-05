## SVC

Supervisor call This instruction causes an exception to be taken to EL1. On executing an SVC instruction, the PE records the exception as a Supervisor Call exception in ESR_ELx, using the EC syndrome value 0x15, and the value of the immediate argument.

#### Decode

let imm : bits(16) = imm16;

### Operation

[AArch64_CheckForSVCTrap](shared_pseudocode.html#func_AArch64_CheckForSVCTrap_1)(imm); [AArch64_CallSupervisor](shared_pseudocode.html#func_AArch64_CallSupervisor_1)(imm);
