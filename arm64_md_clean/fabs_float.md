## FABS (scalar)

Floating-point absolute value (scalar) This instruction calculates the absolute value in the SIMD&FP source register and writes the result to the SIMD&FP destination register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_FP) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '10' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '11' && !IsFeatureImplemented(FEAT_FP16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 8 << UInt(ftype XOR '10');

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); let operand : bits(esize) = V{}(n); var result : bits(128) = if [IsMerging](shared_pseudocode.html#func_IsMerging_1)(FPCR()) then [V](shared_pseudocode.html#accessor_V_2){128}(d) else Zeros{128}; result[0+:esize] = [FPAbs](shared_pseudocode.html#func_FPAbs_3){esize}(operand, FPCR()); [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;
