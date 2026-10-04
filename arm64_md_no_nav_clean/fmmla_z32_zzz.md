## FMMLA (widening, FP16 to FP32)

Half-precision matrix multiply-accumulate to single-precision This instruction multiplies the 2×4 matrix of half-precision values held in each 128-bit segment of the first source vector by the 4×2 matrix of half-precision values in the corresponding segment of the second source vector. The intermediate single-precision results are calculated as a sum of two fused sum-of-products using two pairs of adjacent half-precision values from each of the source vectors, and rounded before accumulation into the 2×2 matrix of single-precision results in the corresponding segment of the destination vector. This is equivalent to performing a 4-way dot product per destination element. This instruction is unpredicated. This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled.

#### Decode

if !IsFeatureImplemented(FEAT_SVE_F16F32MM) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda);

### Operation

[CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let segments : integer = VL DIV 128; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = Z{}(da); var result : bits(VL); for s = 0 to segments-1 do let op1 : bits(128) = operand1[s\*:(128)]; let op2 : bits(128) = operand2[s\*:(128)]; let addend : bits(128) = operand3[s\*:(128)]; result[s\*:(128)] = [FPMatMulAddH](shared_pseudocode.html#func_FPMatMulAddH_5){128}(addend, op1, op2, FPCR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;
