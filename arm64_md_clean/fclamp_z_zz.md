## FCLAMP

Floating-point clamp to minimum/maximum number This instruction clamps each floating-point element in the destination vector to between the floating-point minimum value in the corresponding element of the first source vector and the floating-point maximum value in the corresponding element of the second source vector and destructively places the clamped results in the corresponding elements of the destination vector. Regardless of the value of FPCR.AH, the behavior is as follows for each minimum number and maximum number operation: - Negative zero compares less than positive zero. - If one value is numeric and the other is a quiet NaN, the result is the numeric value. - When FPCR.DN is 0, if either value is a signaling NaN or if both values are NaNs, the result is a quiet NaN. - When FPCR.DN is 1, if either value is a signaling NaN or if both values are NaNs, the result is Default NaN. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SME2) && !IsFeatureImplemented(FEAT_SVE2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; var result : bits(VL); let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = Z{}(d); for e = 0 to elements-1 do let element1 : bits(esize) = operand1[e\*:esize]; let element2 : bits(esize) = operand2[e\*:esize]; let element3 : bits(esize) = operand3[e\*:esize]; let maxelement : bits(esize) = FPMaxNum{}(element1, element3, FPCR()); result[e\*:esize] = [FPMinNum](shared_pseudocode.html#func_FPMinNum_4){esize}(maxelement, element2, FPCR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
