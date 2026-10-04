## BFCVT

Single-precision convert to BFloat16 (scalar) This instruction converts the single-precision floating-point value in the 32-bit SIMD&FP source register to BFloat16 format and writes the result in the 16-bit SIMD&FP destination register. ID_AA64ISAR1_EL1.BF16 indicates whether this instruction is supported.

#### Decode

if !IsFeatureImplemented(FEAT_BF16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn);

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); let operand : bits(32) = V{}(n); let merge : boolean = [IsMerging](shared_pseudocode.html#func_IsMerging_1)(FPCR()); var result : bits(128) = if merge then [V](shared_pseudocode.html#accessor_V_2){128}(d) else Zeros{128}; result[0+:16] = [FPConvertBF](shared_pseudocode.html#func_FPConvertBF_2)(operand, FPCR()); [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;
