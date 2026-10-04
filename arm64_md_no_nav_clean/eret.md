## ERET

Exception return This instruction restores PSTATE from the SPSR, and branches to the address held in the ELR. The SPSR is checked for the current Exception level for an illegal return event. See Illegal exception returns from AArch64 state. ERET is UNDEFINED at EL0.

#### Decode

let pac : boolean = FALSE; let use_key_a : boolean = TRUE;

### Operation

if [PSTATE](shared_pseudocode.html#global_PSTATE).EL == [EL0](shared_pseudocode.html#global_EL0) then Undefined(); end; [AArch64_CheckForERetTrap](shared_pseudocode.html#func_AArch64_CheckForERetTrap_2)(pac, use_key_a); let target : bits(64) = [ELR_ELx](shared_pseudocode.html#accessor_ELR_ELx_0)(); [AArch64_ExceptionReturn](shared_pseudocode.html#func_AArch64_ExceptionReturn_2)(target, [SPSR_ELx](shared_pseudocode.html#accessor_SPSR_ELx_0)());
