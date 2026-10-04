## ERETAA, ERETAB

Exception return, with pointer authentication This instruction authenticates the address in ELR, using SP as the modifier and the specified key, restores PSTATE from the SPSR for the current Exception level, and branches to the authenticated address. Key A is used for ERETAA. Key B is used for ERETAB. If the authentication passes, the PE continues execution at the target of the branch. For information on behavior if the authentication fails, see Faulting on pointer authentication. The authenticated address is not written back to ELR. The SPSR is checked for the current Exception level for an illegal return event. See Illegal exception returns from AArch64 state. ERETAA and ERETAB are UNDEFINED at EL0.

#### Decode

if !IsFeatureImplemented(FEAT_PAuth) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let pac : boolean = TRUE; let use_key_a : boolean = (M == '0'); let auth_then_branch : boolean = TRUE;

### Operation

if [PSTATE](shared_pseudocode.html#global_PSTATE).EL == [EL0](shared_pseudocode.html#global_EL0) then Undefined(); end; [AArch64_CheckForERetTrap](shared_pseudocode.html#func_AArch64_CheckForERetTrap_2)(pac, use_key_a); var target : bits(64) = [ELR_ELx](shared_pseudocode.html#accessor_ELR_ELx_0)(); let modifier : bits(64) = SP{}(); if use_key_a then target = [AuthIA](shared_pseudocode.html#func_AuthIA_3)(target, modifier, auth_then_branch); else target = [AuthIB](shared_pseudocode.html#func_AuthIB_3)(target, modifier, auth_then_branch); end; [AArch64_ExceptionReturn](shared_pseudocode.html#func_AArch64_ExceptionReturn_2)(target, [SPSR_ELx](shared_pseudocode.html#accessor_SPSR_ELx_0)());
