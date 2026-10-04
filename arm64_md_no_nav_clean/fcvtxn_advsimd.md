## FCVTXN, FCVTXN2

Floating-point convert to lower precision narrow, rounding to odd (vector) This instruction reads each double-precision element in the source SIMD&FP register, converts each value to single-precision format using the Round to Odd rounding mode, and writes the results to the destination SIMD&FP register.

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 32; let datasize : integer{} = 64; let elements : integer = 2; let part : integer = UInt(Q);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(2\*datasize) = V{}(n); let merge : boolean = elements == 1 && [IsMerging](shared_pseudocode.html#func_IsMerging_1)(FPCR()); var result : bits(128) = if merge then [V](shared_pseudocode.html#accessor_V_2){128}(d) else Zeros{128}; for e = 0 to elements-1 do result[e\*:esize] = [FPConvert](shared_pseudocode.html#func_FPConvert_5){esize, 2\*esize}(operand[e\*:(2\*esize)], FPCR(), [FPRounding_ODD](shared_pseudocode.html#enum_FPRounding_ODD)); end; if merge then [V](shared_pseudocode.html#accessor_V_2){128}(d) = result; else [Vpart](shared_pseudocode.html#accessor_Vpart_3){datasize}(d, part) = result[0+:datasize]; end;
