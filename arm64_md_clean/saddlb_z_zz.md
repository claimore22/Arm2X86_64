## SADDLB

Signed add long (bottom) This instruction adds the corresponding even-numbered signed elements of the first and second source vectors, and places the results in the overlapping double-width elements of the destination vector. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd); let sel1 : integer = 0; let sel2 : integer = 0; let unsigned : boolean = FALSE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); for e = 0 to elements-1 do let op1elt : bits(esize DIV 2) = operand1[(2\*e + sel1)\*:(esize DIV 2)]; let op2elt : bits(esize DIV 2) = operand2[(2\*e + sel2)\*:(esize DIV 2)]; let element1 : integer = if unsigned then UInt(op1elt) else SInt(op1elt); let element2 : integer = if unsigned then UInt(op2elt) else SInt(op2elt); let res : integer = element1 + element2; result[e\*:esize] = res[esize-1:0]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
