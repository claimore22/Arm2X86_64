## FDOT (4-way, vectors)

8-bit floating-point dot product to single-precision This instruction calculates the fused sum-of-products of a group of four 8-bit floating-point values held in each 32-bit element of the first source and second source vectors. The single-precision sum-of-products are scaled by 2<sup>-UInt(FPMR.LSCALE)</sup>, before being destructively added without intermediate rounding to the corresponding single-precision elements of the destination vector. The 8-bit floating-point encoding format for the elements of the first source vector and the second source vector is selected by FPMR.F8S1 and FPMR.F8S2 respectively. This instruction is unpredicated.

#### Decode

if \![HaveSVE2FP8DOT4](shared_pseudocode.html#func_HaveSVE2FP8DOT4_0)() then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda);

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); if IsFeatureImplemented(FEAT_SSVE_FP8DOT4) && IsFeatureImplemented(FEAT_FP8DOT4) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); elsif IsFeatureImplemented(FEAT_FP8DOT4) then [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); else [CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 32; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = Z{}(da); var result : bits(VL); for e = 0 to elements-1 do let op1 : bits(32) = operand1[e\*:(32)]; let op2 : bits(32) = operand2[e\*:(32)]; var sum : bits(32) = operand3[e\*:(32)]; sum = [FP8DotAddFP](shared_pseudocode.html#func_FP8DotAddFP_7){32, 32}(sum, op1, op2, FPCR(), FPMR()); result[e\*:(32)] = sum; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;
