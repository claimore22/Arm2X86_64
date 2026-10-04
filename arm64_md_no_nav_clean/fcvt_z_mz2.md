## FCVT (narrowing, FP32 to FP16)

Multi-vector single-precision convert to half-precision This instruction converts each element of the two source vectors from single-precision to half-precision floating-point, and places the results in the half-width destination elements. This instruction follows SME2 floating-point numerical behaviors corresponding to instructions that place their results in one or more SVE Z vectors. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn::'0'); let d : integer = UInt(Zd);

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 32; var result : bits(VL); let operand1 : bits(VL) = Z{}(n+0); let operand2 : bits(VL) = Z{}(n+1); for e = 0 to elements-1 do let element1 : bits(32) = operand1[e\*:32]; let element2 : bits(32) = operand2[e\*:32]; let res1 : bits(16) = [FPConvertSVE](shared_pseudocode.html#func_FPConvertSVE_4){16, 32}(element1, FPCR()); let res2 : bits(16) = [FPConvertSVE](shared_pseudocode.html#func_FPConvertSVE_4){16, 32}(element2, FPCR()); result[e\*:16] = res1; result[(elements+e)\*:16] = res2; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
