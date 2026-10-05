## UMAX (immediate)

Unsigned maximum with immediate (unpredicated) This instruction determines the unsigned maximum of an immediate and each element of the source vector, and destructively places the results in the corresponding elements of the source vector. The immediate is an unsigned 8-bit value in the range 0 to 255, inclusive. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let dn : integer = UInt(Zdn); let unsigned : boolean = TRUE; let imm : integer = if unsigned then UInt(imm8) else SInt(imm8);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(dn); var result : bits(VL); for e = 0 to elements-1 do let op1elt : bits(esize) = operand1[e\*:esize]; let element1 : integer = if unsigned then UInt(op1elt) else SInt(op1elt); result[e\*:esize] = Max(element1, imm)[esize-1:0]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;
