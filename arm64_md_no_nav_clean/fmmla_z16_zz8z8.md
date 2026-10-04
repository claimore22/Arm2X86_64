## FMMLA (widening, FP8 to FP16)

8-bit floating-point matrix multiply-accumulate to half-precision This instruction multiplies the 2×4 matrix of 8-bit floating-point values held in each 64-bit segment of the first source vector by the 4×2 matrix of 8-bit floating-point values in the corresponding segment of the second source vector. The intermediate half-precision results calculated as a fused sum-of-products using four adjacent 8-bit floating-point values from each of the source vectors, are scaled by 2<sup>-UInt(FPMR.LSCALE[3:0])</sup>, before being destructively added without intermediate rounding to the 2×2 half-precision matrix in the corresponding segment of the destination vector. This is equivalent to accumulating 4-way dot product per destination element. The 8-bit floating-point encoding format for the elements of the first source vector and the second source vector is selected by FPMR.F8S1 and FPMR.F8S2 respectively. This instruction is unpredicated. This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) \|\| !IsFeatureImplemented(FEAT_F8F16MM) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda);

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let segments : integer = VL DIV 64; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = Z{}(da); var result : bits(VL); for s = 0 to segments-1 do let op1 : bits(64) = operand1[s\*:(64)]; let op2 : bits(64) = operand2[s\*:(64)]; let addend : bits(64) = operand3[s\*:(64)]; let way : integer{} = 4; result[s\*:(64)] = [FP8MatMulAddFP](shared_pseudocode.html#func_FP8MatMulAddFP_7){64}(addend, op1, op2, way, FPCR(), FPMR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;
