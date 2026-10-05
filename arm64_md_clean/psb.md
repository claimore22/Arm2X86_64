## PSB

Profiling synchronization barrier This instruction is a barrier that ensures that all existing profiling data for the current PE has been formatted, and profiling buffer addresses have been translated such that all writes to the profiling buffer have been initiated. A following DSB instruction completes when the writes to the profiling buffer have completed. If FEAT_SPE is not implemented, this instruction executes as a NOP.

#### Decode

if !IsFeatureImplemented(FEAT_SPE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end;

### Operation

if IsFeatureImplemented(FEAT_FGT) && IsFeatureImplemented(FEAT_SPEv1p5) then let trap_to_el2 : boolean = ([PSTATE](shared_pseudocode.html#global_PSTATE).EL IN {[EL0](shared_pseudocode.html#global_EL0), [EL1](shared_pseudocode.html#global_EL1)} && [EL2Enabled](shared_pseudocode.html#func_EL2Enabled_0)() && \![IsInHost](shared_pseudocode.html#func_IsInHost_0)() && (\![HaveEL](shared_pseudocode.html#func_HaveEL_1)([EL3](shared_pseudocode.html#global_EL3)) \|\| SCR_EL3().FGTEn == '1') && HFGITR_EL2().PSBCSYNC == '1'); if trap_to_el2 then let target_el : bits(2) = [EL2](shared_pseudocode.html#global_EL2); let iss : bits(25) = 0x3[24:0]; [AArch64_OtherInstrTrap](shared_pseudocode.html#func_AArch64_OtherInstrTrap_2)(target_el, iss); end; end; ProfilingSynchronizationBarrier();
