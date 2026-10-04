## FMLALLBB, FMLALLBT, FMLALLTB, FMLALLTT (by element)

8-bit floating-point multiply-add to single-precision (vector, by element) This instruction widens the first (bottom bottom), second (bottom top), third (top bottom), or fourth (top top) 8-bit element of each 32-bit container in the first source vector and the indexed element from the second source vector to single-precision format and multiplies the corresponding elements. The intermediate products are scaled by 2<sup>-UInt(FPMR.LSCALE)</sup>, before being destructively added without intermediate rounding to the single-precision elements of the destination vector that overlap with the corresponding 8-bit floating-point elements in the first source vector. The 8-bit floating-point encoding format for the elements of the first source vector is selected by FPMR.F8S1. The 8-bit floating-point encoding format for the elements of the second source vector is selected by FPMR.F8S2.

#### Decode

if !IsFeatureImplemented(FEAT_FP8FMA) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Rn); let m : integer = UInt('00'::Rm[2:0]); let d : integer = UInt(Rd); let index : integer = UInt(H::L::M::Rm[3]); let elements : integer = 128 DIV 32; let sel : integer = UInt(Q::size[0]);

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); [AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(128) = V{}(n); let operand2 : bits(128) = V{}(m); let operand3 : bits(128) = V{}(d); var result : bits(128); for e = 0 to elements-1 do let element1 : bits(8) = operand1[(4 \* e + sel)\*:8]; let element2 : bits(8) = operand2[index\*:8]; let element3 : bits(32) = operand3[e\*:32]; result[e\*:32] = [FP8MulAddFP](shared_pseudocode.html#func_FP8MulAddFP_6){32}(element3, element1, element2, FPCR(), FPMR()); end; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;
