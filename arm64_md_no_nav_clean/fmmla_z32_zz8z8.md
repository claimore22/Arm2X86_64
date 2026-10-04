## FMMLA (widening, FP8 to FP32)

8-bit floating-point matrix multiply-accumulate to single-precision This instruction multiplies the 2×8 matrix of 8-bit floating-point values held in each 128-bit segment of the first source vector by the 8×2 matrix of 8-bit floating-point values in the corresponding segment of the second source vector. The intermediate single-precision results calculated as a fused sum-of-products using eight adjacent 8-bit floating-point values from each of the source vectors, are scaled by 2<sup>-UInt(FPMR.LSCALE)</sup>, before being destructively added without intermediate rounding to the 2×2 single-precision matrix in the corresponding segment of the destination vector. This is equivalent to accumulating 8-way dot product per destination element. The 8-bit floating-point encoding format for the elements of the first source vector and the second source vector is selected by FPMR.F8S1 and FPMR.F8S2 respectively. This instruction is unpredicated. This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) \|\| !IsFeatureImplemented(FEAT_F8F32MM) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda);

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let segments : integer = VL DIV 128; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = Z{}(da); var result : bits(VL); for s = 0 to segments-1 do let op1 : bits(128) = operand1[s\*:(128)]; let op2 : bits(128) = operand2[s\*:(128)]; let addend : bits(128) = operand3[s\*:(128)]; let way : integer{} = 8; result[s\*:(128)] = [FP8MatMulAddFP](shared_pseudocode.html#func_FP8MatMulAddFP_7){128}(addend, op1, op2, way, FPCR(), FPMR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;
