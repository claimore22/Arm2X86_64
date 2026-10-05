## FMLALLTT (vectors)

8-bit floating-point multiply-add to single-precision (top top) This instruction widens the fourth 8-bit element of each 32-bit container in the first and second source vectors to single-precision format and multiplies the corresponding elements. The intermediate products are scaled by 2<sup>-UInt(FPMR.LSCALE)</sup> before being destructively added without intermediate rounding to the single-precision elements of the destination vector that overlap with the corresponding 8-bit floating-point elements in the source vectors. The 8-bit floating-point encoding format for the elements of the first source vector and the second source vector is selected by FPMR.F8S1 and FPMR.F8S2 respectively. This instruction is unpredicated.

#### Decode

if \![HaveSVE2FP8FMA](shared_pseudocode.html#func_HaveSVE2FP8FMA_0)() then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda);

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); if IsFeatureImplemented(FEAT_SSVE_FP8FMA) && IsFeatureImplemented(FEAT_FP8FMA) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); elsif IsFeatureImplemented(FEAT_FP8FMA) then [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); else [CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 32; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = Z{}(da); var result : bits(VL); for e = 0 to elements-1 do let element1 : bits(8) = operand1[(4 \* e + 3)\*:8]; let element2 : bits(8) = operand2[(4 \* e + 3)\*:8]; let element3 : bits(32) = operand3[e\*:32]; result[e\*:32] = [FP8MulAddFP](shared_pseudocode.html#func_FP8MulAddFP_6){32}(element3, element1, element2, FPCR(), FPMR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;
