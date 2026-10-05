## FMLALB (vectors, FP16 to FP32)

Half-precision multiply-add to single-precision (bottom) This instruction widens the even-numbered half-precision elements in the first source vector and the corresponding elements in the second source vector to single-precision format and then destructively multiplies and adds these values without intermediate rounding to the single-precision elements of the destination vector that overlap with the corresponding half-precision elements in the source vectors. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda); let op1_neg : boolean = FALSE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let op1 : bits(VL) = Z{}(n); let op2 : bits(VL) = Z{}(m); let op3 : bits(VL) = Z{}(da); var result : bits(VL); for e = 0 to elements-1 do let elem1 : bits(esize DIV 2) = (if op1_neg then [FPNeg](shared_pseudocode.html#func_FPNeg_3){esize DIV 2}(op1[(2\*e + 0)\*:(esize DIV 2)], FPCR()) else op1[(2\*e + 0)\*:(esize DIV 2)]); let elem2 : bits(esize DIV 2) = op2[(2\*e + 0)\*:(esize DIV 2)]; let elem3 : bits(esize) = op3[e\*:esize]; result[e\*:esize] = [FPMulAddH](shared_pseudocode.html#func_FPMulAddH_4)(elem3, elem1, elem2, FPCR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;
