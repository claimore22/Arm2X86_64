## FCSEL

Floating-point conditional select (scalar) This instruction allows the SIMD&FP destination register to take the value from either one or the other of two SIMD&FP source registers. If the condition passes, the first SIMD&FP source register value is taken, otherwise the second SIMD&FP source register value is taken. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_FP) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '10' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '11' && !IsFeatureImplemented(FEAT_FP16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 8 << UInt(ftype XOR '10'); let condition : bits(4) = cond;

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); var result : bits(datasize); let condition_holds : boolean = [ConditionHolds](shared_pseudocode.html#func_ConditionHolds_1)(condition); result = if condition_holds then [V](shared_pseudocode.html#accessor_V_2){datasize}(n) else [V](shared_pseudocode.html#accessor_V_2){datasize}(m); [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;
