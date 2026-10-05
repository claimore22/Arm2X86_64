## FMMLA (widening, 8-bit floating-point to single-precision)

8-bit floating-point matrix multiply-accumulate to single-precision This instruction multiplies the 2×8 matrix of 8-bit floating-point values held in the first source vector by the 8×2 matrix of 8-bit floating-point values in the second source vector. The intermediate single-precision results, calculated as a fused sum-of-products using eight adjacent 8-bit floating-point values from each of the source vectors, are scaled by 2<sup>-UInt(FPMR.LSCALE)</sup> before being destructively added without intermediate rounding to the 2x2 single-precision matrix in the destination vector. This is equivalent to accumulating 8-way dot product per destination element. The 8-bit floating-point encoding format for the elements of the first source vector is selected by FPMR.F8S1. The 8-bit floating-point encoding format for the elements of the second source vector is selected by FPMR.F8S2.

#### Decode

if !IsFeatureImplemented(FEAT_F8F32MM) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Rn); let m : integer = UInt(Rm); let d : integer = UInt(Rd);

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); [AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let op1 : bits(128) = V{}(n); let op2 : bits(128) = V{}(m); let acc : bits(128) = V{}(d); [V](shared_pseudocode.html#accessor_V_2){128}(d) = [FP8MatMulAddFP](shared_pseudocode.html#func_FP8MatMulAddFP_7){128}(acc, op1, op2, 8, FPCR(), FPMR());
