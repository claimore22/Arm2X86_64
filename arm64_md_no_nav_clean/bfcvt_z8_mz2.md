## BFCVT (BFloat16 to 8-bit floating-point)

Multi-vector BFloat16 convert to 8-bit floating-point This instruction converts each BFloat16 element of the two source vectors to 8-bit floating-point while scaling the value by 2<sup>SInt(FPMR.NSCALE)</sup>, and places the results in the half-width elements of the destination vector. The 8-bit floating-point encoding format is selected by FPMR.F8D. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SME2) \|\| !IsFeatureImplemented(FEAT_FP8) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn::'0'); let d : integer = UInt(Zd);

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); [CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 16; var result : bits(VL); let operand1 : bits(VL) = Z{}(n+0); let operand2 : bits(VL) = Z{}(n+1); for e = 0 to elements-1 do let element1 : bits(16) = operand1[e\*:16]; let element2 : bits(16) = operand2[e\*:16]; result[(0\*elements + e)\*:8] = [BFConvertFP8](shared_pseudocode.html#func_BFConvertFP8_3)(element1, FPCR(), FPMR()); result[(1\*elements + e)\*:8] = [BFConvertFP8](shared_pseudocode.html#func_BFConvertFP8_3)(element2, FPCR(), FPMR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
