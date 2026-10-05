## BFMLSLT (vectors)

BFloat16 multiply-subtract from single-precision (top) This instruction widens the odd-numbered BFloat16 elements in the first source vector and the corresponding elements in the second source vector to single-precision format and then destructively multiplies and subtracts these values without intermediate rounding from the single-precision elements of the destination vector that overlap with the corresponding BFloat16 elements in the source vectors. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SME2) && !IsFeatureImplemented(FEAT_SVE2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda); let op1_neg : boolean = TRUE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 32; let op1 : bits(VL) = Z{}(n); let op2 : bits(VL) = Z{}(m); let op3 : bits(VL) = Z{}(da); var result : bits(VL); for e = 0 to elements-1 do let elem1 : bits(16) = (if op1_neg then [BFNeg](shared_pseudocode.html#func_BFNeg_1)(op1[(2\*e + 1)\*:16]) else op1[(2\*e + 1)\*:16]); let elem2 : bits(16) = op2[(2\*e + 1)\*:16]; let elem3 : bits(32) = op3[e\*:32]; result[e\*:32] = [BFMulAddH](shared_pseudocode.html#func_BFMulAddH_4)(elem3, elem1, elem2, FPCR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;
