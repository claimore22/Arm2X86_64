## TSB

Trace synchronization barrier This instruction is a barrier that synchronizes the trace operations of instructions, see Trace Synchronization Barrier (TSB). If FEAT_TRF is not implemented, this instruction executes as a NOP.

#### Decode

if !IsFeatureImplemented(FEAT_TRF) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end;

### Operation

if IsFeatureImplemented(FEAT_FGT2) && IsFeatureImplemented(FEAT_TRBEv1p1) then let trap_to_el2 : boolean = ([PSTATE](shared_pseudocode.html#global_PSTATE).EL IN {[EL0](shared_pseudocode.html#global_EL0), [EL1](shared_pseudocode.html#global_EL1)} && [EL2Enabled](shared_pseudocode.html#func_EL2Enabled_0)() && \![IsInHost](shared_pseudocode.html#func_IsInHost_0)() && (\![HaveEL](shared_pseudocode.html#func_HaveEL_1)([EL3](shared_pseudocode.html#global_EL3)) \|\| SCR_EL3().FGTEn2 == '1') && HFGITR2_EL2().TSBCSYNC == '1'); if trap_to_el2 then let target_el : bits(2) = [EL2](shared_pseudocode.html#global_EL2); let iss : bits(25) = 0x4[24:0]; [AArch64_OtherInstrTrap](shared_pseudocode.html#func_AArch64_OtherInstrTrap_2)(target_el, iss); end; end; [TraceSynchronizationBarrier](shared_pseudocode.html#func_TraceSynchronizationBarrier_0)();
