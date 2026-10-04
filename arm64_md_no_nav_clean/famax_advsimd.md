## FAMAX

Floating-point absolute maximum This instruction determines the maximum absolute value from floating-point elements of the first source vector and the corresponding floating-point elements of the second source vector, and places the results in the corresponding elements of the destination vector. Regardless of the value of FPCR.AH, the behavior is as follows: - When FPCR.DN is 0, if either element is a NaN, the result is a quiet NaN. - When FPCR.DN is 1, if either element is a NaN, the result is the Default NaN, with the sign bit set to 0. - Denormalized inputs and results are never flushed to zero, as if FPCR.{FZ, FZ16, FIZ} are all 0. - Denormalized inputs never generate an Input Denormal floating-point exception. It has encodings from 2 classes: [Half-precision](#iclass_half_precision) and [Single-precision and double-precision](#iclass_single_precision_and_double_precision)

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) \|\| !IsFeatureImplemented(FEAT_FAMINMAX) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if Q == '0' && size == '11' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let esize : integer{} = 8 << UInt(size); let datasize : integer{} = if Q == '1' then 128 else 64; let elements : integer = datasize DIV esize;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(datasize) = V{}(n); let operand2 : bits(datasize) = V{}(m); var result : bits(datasize); for e = 0 to elements-1 do let op1 : bits(esize) = operand1[e\*:esize]; let op2 : bits(esize) = operand2[e\*:esize]; result[e\*:esize] = [FPAbsMax](shared_pseudocode.html#func_FPAbsMax_4){esize}(op1, op2, FPCR()); end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;
