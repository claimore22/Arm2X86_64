## SABA

Signed absolute difference and accumulate This instruction calculates the absolute difference between signed integer values in elements of the first source vector and the corresponding elements of the second source vector, and adds the difference to the corresponding elements of the destination vector. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda); let unsigned : boolean = FALSE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL) = Z{}(da); for e = 0 to elements-1 do let op1elt : bits(esize) = operand1[e\*:esize]; let op2elt : bits(esize) = operand2[e\*:esize]; let element1 : integer = if unsigned then UInt(op1elt) else SInt(op1elt); let element2 : integer = if unsigned then UInt(op2elt) else SInt(op2elt); let absdiff : bits(esize) = Abs(element1 - element2)[esize-1:0]; result[e\*:esize] = result[e\*:esize] + absdiff; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;
