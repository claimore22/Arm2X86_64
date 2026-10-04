## BFMLA (indexed)

BFloat16 fused multiply-add by indexed element This instruction multiplies all BFloat16 elements within each 128-bit segment of the first source vector by the specified element in the corresponding second source vector segment. The products are then destructively added without intermediate rounding to the corresponding elements of the addend and destination vector. The elements within the second source vector are specified using an immediate index that selects the same element position within each 128-bit vector segment. The index range is from 0 to 7. This instruction follows SVE2 non-widening BFloat16 numerical behaviors. This instruction is unpredicated. ID_AA64ZFR0_EL1.B16B16 indicates whether this instruction is implemented.

#### Decode

if !IsFeatureImplemented(FEAT_SVE_B16B16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let index : integer = UInt(i3h::i3l); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda); let op1_neg : boolean = FALSE; let op3_neg : boolean = FALSE;

### Operation

if IsFeatureImplemented(FEAT_SME2) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 16; let eltspersegment : integer = 128 DIV 16; let op1 : bits(VL) = Z{}(n); let op2 : bits(VL) = Z{}(m); var result : bits(VL) = Z{}(da); for e = 0 to elements-1 do let segmentbase : integer = e - (e MOD eltspersegment); let s : integer = segmentbase + index; let elem2 : bits(16) = op2[s\*:16]; let elem1 : bits(16) = if op1_neg then [BFNeg](shared_pseudocode.html#func_BFNeg_1)(op1[e\*:16]) else op1[e\*:16]; let elem3 : bits(16) = if op3_neg then [BFNeg](shared_pseudocode.html#func_BFNeg_1)(result[e\*:16]) else result[e\*:16]; result[e\*:16] = [BFMulAdd](shared_pseudocode.html#func_BFMulAdd_4)(elem3, elem1, elem2, FPCR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;
