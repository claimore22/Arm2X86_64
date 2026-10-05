## BFCLAMP

BFloat16 clamp to minimum/maximum number This instruction clamps each BFloat16 element in the destination vector to between the BFloat16 minimum value in the corresponding element of the first source vector and the BFloat16 maximum value in the corresponding element of the second source vector and destructively places the clamped results in the corresponding elements of the destination vector. Regardless of the value of FPCR.AH, the behavior is as follows for each minimum number and maximum number operation: - Negative zero compares less than positive zero. - If one value is numeric and the other is a quiet NaN, the result is the numeric value. - When FPCR.DN is 0, if either value is a signaling NaN or if both values are NaNs, the result is a quiet NaN. - When FPCR.DN is 1, if either value is a signaling NaN or if both values are NaNs, the result is Default NaN. This instruction follows SVE2 non-widening BFloat16 numerical behaviors. This instruction is unpredicated. ID_AA64ZFR0_EL1.B16B16 indicates whether this instruction is implemented.

#### Decode

if !IsFeatureImplemented(FEAT_SVE_B16B16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd);

### Operation

if IsFeatureImplemented(FEAT_SME2) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 16; var result : bits(VL); let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = Z{}(d); for e = 0 to elements-1 do let element1 : bits(16) = operand1[e\*:16]; let element2 : bits(16) = operand2[e\*:16]; let element3 : bits(16) = operand3[e\*:16]; let maxelement : bits(16) = BFMaxNum{}(element1, element3, FPCR()); result[e\*:16] = [BFMinNum](shared_pseudocode.html#func_BFMinNum_4){16}(maxelement, element2, FPCR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
