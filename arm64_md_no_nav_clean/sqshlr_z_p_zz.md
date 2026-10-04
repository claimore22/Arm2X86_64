## SQSHLR

Signed saturating shift left reversed (predicated) This instruction shifts active signed elements of the second source vector by the corresponding elements of the first source vector, and destructively places the results in the corresponding elements of the first source vector. A positive shift amount performs a left shift, otherwise a right shift by the negated shift amount is performed. Each result element is saturated to the N-bit element's signed integer range -2<sup>(N-1)</sup> to (2<sup>(N-1)</sup>)-1. Inactive elements in the destination vector register remain unmodified.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let m : integer = UInt(Zm); let dn : integer = UInt(Zdn);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand1 : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(m) else Zeros{VL}; let operand2 : bits(VL) = Z{}(dn); var result : bits(VL); for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then let element : integer = SInt(operand1[e\*:esize]); var shift : integer = [ShiftSat](shared_pseudocode.html#func_ShiftSat_2)(SInt(operand2[e\*:esize]), esize); var res : integer; if shift >= 0 then res = element << shift; else shift = -shift; res = element >> shift; end; result[e\*:esize] = [SignedSat](shared_pseudocode.html#func_SignedSat_2){esize}(res); else result[e\*:esize] = operand2[e\*:esize]; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;
