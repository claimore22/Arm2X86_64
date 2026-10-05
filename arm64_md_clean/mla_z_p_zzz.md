## MLA (vectors)

Multiply-add (predicated) This instruction multiplies the active elements of the first source vector by the corresponding elements of the second source vector, and adds the result to elements of the third source (addend) vector. The results are destructively placed in the destination and third source (addend) vector. Inactive elements in the destination vector register remain unmodified.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda); let sub_op : boolean = FALSE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand1 : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(n) else Zeros{VL}; let operand2 : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(m) else Zeros{VL}; let operand3 : bits(VL) = Z{}(da); var result : bits(VL); for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then let element1 : integer = UInt(operand1[e\*:esize]); let element2 : integer = UInt(operand2[e\*:esize]); let product : integer = element1 \* element2; if sub_op then result[e\*:esize] = operand3[e\*:esize] - product; else result[e\*:esize] = operand3[e\*:esize] + product; end; else result[e\*:esize] = operand3[e\*:esize]; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;
