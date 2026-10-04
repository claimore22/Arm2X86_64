## FMMLA (widening, 8-bit floating-point to half-precision)

8-bit floating-point matrix multiply-accumulate to half-precision This instruction multiplies the 2×4 matrix of 8-bit floating-point values held in each 64-bit segment of the first source vector by the 4×2 matrix of 8-bit floating-point values in the corresponding segment of the second source vector. The intermediate half-precision results, calculated as a fused sum-of-products using four adjacent 8-bit floating-point values from each of the source vectors, are scaled by 2<sup>-UInt(FPMR.LSCALE[3:0])</sup> before being destructively added without intermediate rounding to the 2x2 half-precision matrix in the corresponding segment of the destination vector. This is equivalent to accumulating 4-way dot product per destination element. The 8-bit floating-point encoding format for the elements of the first source vector is selected by FPMR.F8S1. The 8-bit floating-point encoding format for the elements of the second source vector is selected by FPMR.F8S2.

#### Decode

if !IsFeatureImplemented(FEAT_F8F16MM) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Rn); let m : integer = UInt(Rm); let d : integer = UInt(Rd);

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); [AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(128) = V{}(n); let operand2 : bits(128) = V{}(m); let operand3 : bits(128) = V{}(d); var result : bits(128); var op1, op2, acc : bits(64); for s = 0 to 1 do op1 = operand1[s\*:64]; op2 = operand2[s\*:64]; acc = operand3[s\*:64]; result[s\*:64] = [FP8MatMulAddFP](shared_pseudocode.html#func_FP8MatMulAddFP_7){64}(acc, op1, op2, 4, FPCR(), FPMR()); end; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;
