## FMINNMV

Floating-point minimum number recursive reduction to scalar This instruction calculates a floating-point minimum number horizontally over all lanes of a vector using a recursive pairwise reduction, and places the result in the SIMD&FP scalar destination register. Inactive elements in the source vector are treated as the default NaN. Regardless of the value of FPCR.AH, the behavior is as follows: - Negative zero compares less than positive zero. - If one value is numeric and the other is a quiet NaN, the result is the numeric value. - When FPCR.DN is 0, if either value is a signaling NaN or if both values are NaNs, the result is a quiet NaN. - When FPCR.DN is 1, if either value is a signaling NaN or if both values are NaNs, the result is Default NaN.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Vd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let mask : bits(PL) = P{}(g); let operand : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(n) else Zeros{VL}; let identity : bits(esize) = [FPDefaultNaN](shared_pseudocode.html#func_FPDefaultNaN_2){esize}(FPCR()); [V](shared_pseudocode.html#accessor_V_2){esize}(d) = [FPReducePredicated](shared_pseudocode.html#func_FPReducePredicated_8){esize, VL, PL}([ReduceOp_FMINNUM](shared_pseudocode.html#enum_ReduceOp_FMINNUM), operand, mask, identity, FPCR());
