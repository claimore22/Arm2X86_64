## FCVTN

Half-precision convert to interleaved 8-bit floating-point This instruction converts each half-precision element of the pair of source vectors to 8-bit floating-point while scaling the value by 2<sup>SInt(FPMR.NSCALE[4:0])</sup>, and places the two-way interleaved results in the corresponding 8-bit elements of the destination vector. The 8-bit floating-point encoding format is selected by FPMR.F8D. This instruction is unpredicated.

#### Decode

if ((!IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME2)) \|\| !IsFeatureImplemented(FEAT_FP8)) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn::'0'); let d : integer = UInt(Zd);

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); if IsFeatureImplemented(FEAT_SME2) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 16; var result : bits(VL); let operand1 : bits(VL) = Z{}(n+0); let operand2 : bits(VL) = Z{}(n+1); for e = 0 to elements-1 do let element1 : bits(16) = operand1[e\*:16]; let element2 : bits(16) = operand2[e\*:16]; result[(2\*e + 0)\*:8] = [FPConvertFP8](shared_pseudocode.html#func_FPConvertFP8_5){8, 16}(element1, FPCR(), FPMR()); result[(2\*e + 1)\*:8] = [FPConvertFP8](shared_pseudocode.html#func_FPConvertFP8_5){8, 16}(element2, FPCR(), FPMR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
