## FMAXV

Floating-point maximum recursive reduction to scalar This instruction calculates a floating-point maximum horizontally over all lanes of a vector using a recursive pairwise reduction, and places the result in the SIMD&FP scalar destination register. Inactive elements in the source vector are treated as -Infinity. When FPCR.AH is 0, the behavior is as follows: - Negative zero compares less than positive zero. - When FPCR.DN is 0, if either value is a NaN, the result is a quiet NaN. - When FPCR.DN is 1, if either value is a NaN, the result is Default NaN. When FPCR.AH is 1, the behavior is as follows: - If both values are zeros, regardless of the sign of either zero, the result is the second value. - If either value is a NaN, regardless of the value of FPCR.DN, the result is the second value.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Vd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let mask : bits(PL) = P{}(g); let operand : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(n) else Zeros{VL}; let identity : bits(esize) = [FPInfinity](shared_pseudocode.html#func_FPInfinity_2){esize}('1'); [V](shared_pseudocode.html#accessor_V_2){esize}(d) = [FPReducePredicated](shared_pseudocode.html#func_FPReducePredicated_8){esize, VL, PL}([ReduceOp_FMAX](shared_pseudocode.html#enum_ReduceOp_FMAX), operand, mask, identity, FPCR());
