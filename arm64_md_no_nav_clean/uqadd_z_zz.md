## UQADD (vectors, unpredicated)

Unsigned saturating add (unpredicated) This instruction performs an unsigned saturating addition of all elements of the first source vector to the corresponding elements of the second source vector, and places the results in the corresponding elements of the destination vector. Each result element is saturated to the N-bit element's unsigned integer range 0 to (2<sup>N</sup>)-1. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd); let unsigned : boolean = TRUE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); for e = 0 to elements-1 do let op1elt : bits(esize) = operand1[e\*:esize]; let op2elt : bits(esize) = operand2[e\*:esize]; if unsigned then let element1 : integer = UInt(op1elt); let element2 : integer = UInt(op2elt); (result[e\*:esize], -) = [UnsignedSatQ](shared_pseudocode.html#func_UnsignedSatQ_2){esize}(element1 + element2); else let element1 : integer = SInt(op1elt); let element2 : integer = SInt(op2elt); (result[e\*:esize], -) = [SignedSatQ](shared_pseudocode.html#func_SignedSatQ_2){esize}(element1 + element2); end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
