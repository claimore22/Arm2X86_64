## BFMINNM (multiple vectors)

Multi-vector BFloat16 minimum number This instruction determines the minimum number value of BFloat16 elements of the two or four second source vectors and the corresponding BFloat16 elements of the two or four first source vectors and destructively places the results in the corresponding elements of the two or four first source vectors. Regardless of the value of FPCR.AH, the behavior is as follows: - Negative zero compares less than positive zero. - If one element is numeric and the other is a quiet NaN, the result is the numeric value. - When FPCR.DN is 0, if either element is a signaling NaN or if both elements are NaNs, the result is a quiet NaN. - When FPCR.DN is 1, if either element is a signaling NaN or if both elements are NaNs, the result is Default NaN. This instruction follows SME2 non-widening BFloat16 numerical behaviors corresponding to instructions that place their results in two or four SVE Z vectors. This instruction is unpredicated. ID_AA64ZFR0_EL1.B16B16 indicates whether this instruction is implemented. It has encodings from 2 classes: [Two registers](#iclass_two_registers) and [Four registers](#iclass_four_registers)

#### Decode

if !IsFeatureImplemented(FEAT_SME2) \|\| !IsFeatureImplemented(FEAT_SVE_B16B16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let dn : integer = UInt(Zdn::'00'); let m : integer = UInt(Zm::'00'); let nreg : integer{} = 4;

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 16; var results : array [[4]] of bits(VL); for r = 0 to nreg-1 do let operand1 : bits(VL) = Z{}(dn+r); let operand2 : bits(VL) = Z{}(m+r); for e = 0 to elements-1 do let element1 : bits(16) = operand1[e\*:16]; let element2 : bits(16) = operand2[e\*:16]; results[[r]][e\*:16] = [BFMinNum](shared_pseudocode.html#func_BFMinNum_4){16}(element1, element2, FPCR()); end; end; for r = 0 to nreg-1 do [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn+r) = results[[r]]; end;
