## FMIN (immediate)

Floating-point minimum with immediate (predicated) This instruction determines the minimum of an immediate and each active floating-point element of the source vector, and destructively places the results in the corresponding elements of the source vector. The immediate may take the value +0.0 or +1.0 only. When FPCR.AH is 0, the behavior is as follows: - Negative zero compares less than positive zero. - When FPCR.DN is 0, if the element is a NaN, the result is a quiet NaN. - When FPCR.DN is 1, if the element is a NaN, the result is Default NaN. When FPCR.AH is 1, the behavior is as follows: - If both the element and the immediate are zeros, regardless of the sign of either zero, the result is the immediate. - If the element is a NaN, regardless of the value of FPCR.DN, the result is the immediate. Inactive elements in the destination vector register remain unmodified.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let dn : integer = UInt(Zdn); let imm : bits(esize) = if i1 == '0' then Zeros{esize} else [FPOne](shared_pseudocode.html#func_FPOne_2){esize}('0');

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand1 : bits(VL) = Z{}(dn); var result : bits(VL); for e = 0 to elements-1 do let element1 : bits(esize) = operand1[e\*:esize]; if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then result[e\*:esize] = [FPMin](shared_pseudocode.html#func_FPMin_4){esize}(element1, imm, FPCR()); else result[e\*:esize] = element1; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;
