## FMAXNM (vectors)

Floating-point maximum number (predicated) This instruction determines the maximum number value of active floating-point elements of the first source vector and the corresponding floating-point elements of the second source vector, and destructively places the results in the corresponding elements of the first source vector. Regardless of the value of FPCR.AH, the behavior is as follows: - Negative zero compares less than positive zero. - If one element is numeric and the other is a quiet NaN, the result is the numeric value. - When FPCR.DN is 0, if either element is a signaling NaN or if both elements are NaNs, the result is a quiet NaN. - When FPCR.DN is 1, if either element is a signaling NaN or if both elements are NaNs, the result is Default NaN. Inactive elements in the destination vector register remain unmodified.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let dn : integer = UInt(Zdn); let m : integer = UInt(Zm);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand1 : bits(VL) = Z{}(dn); let operand2 : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(m) else Zeros{VL}; var result : bits(VL); for e = 0 to elements-1 do let element1 : bits(esize) = operand1[e\*:esize]; if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then let element2 : bits(esize) = operand2[e\*:esize]; result[e\*:esize] = [FPMaxNum](shared_pseudocode.html#func_FPMaxNum_4){esize}(element1, element2, FPCR()); else result[e\*:esize] = element1; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;
