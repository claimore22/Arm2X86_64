## FCVTN (FP32 to FP8)

Multi-vector single-precision convert to interleaved 8-bit floating-point This instruction converts each single-precision element of the four source vectors to 8-bit floating-point while scaling the value by 2<sup>SInt(FPMR.NSCALE)</sup>, and places the four-way interleaved results in the quarter-width elements of the destination vector. The 8-bit floating-point encoding format is selected by FPMR.F8D. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SME2) \|\| !IsFeatureImplemented(FEAT_FP8) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn::'00'); let d : integer = UInt(Zd);

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); [CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 32; var result : bits(VL); let operand1 : bits(VL) = Z{}(n+0); let operand2 : bits(VL) = Z{}(n+1); let operand3 : bits(VL) = Z{}(n+2); let operand4 : bits(VL) = Z{}(n+3); for e = 0 to elements-1 do let element1 : bits(32) = operand1[e\*:32]; let element2 : bits(32) = operand2[e\*:32]; let element3 : bits(32) = operand3[e\*:32]; let element4 : bits(32) = operand4[e\*:32]; result[(4\*e + 0)\*:8] = [FPConvertFP8](shared_pseudocode.html#func_FPConvertFP8_5){8, 32}(element1, FPCR(), FPMR()); result[(4\*e + 1)\*:8] = [FPConvertFP8](shared_pseudocode.html#func_FPConvertFP8_5){8, 32}(element2, FPCR(), FPMR()); result[(4\*e + 2)\*:8] = [FPConvertFP8](shared_pseudocode.html#func_FPConvertFP8_5){8, 32}(element3, FPCR(), FPMR()); result[(4\*e + 3)\*:8] = [FPConvertFP8](shared_pseudocode.html#func_FPConvertFP8_5){8, 32}(element4, FPCR(), FPMR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
