## BF1CVTLT, BF2CVTLT

8-bit floating-point convert to BFloat16 (top) This instruction converts each odd-numbered 8-bit floating-point element of the source vector to BFloat16 while downscaling the value, and places the results in the overlapping 16-bit elements of the destination vector. BF1CVTLT scales the values by 2<sup>-UInt(FPMR.LSCALE[5:0])</sup>. BF2CVTLT scales the values by 2<sup>-UInt(FPMR.LSCALE2[5:0])</sup>. The 8-bit floating-point encoding format for BF1CVTLT is selected by FPMR.F8S1. The 8-bit floating-point encoding format for BF2CVTLT is selected by FPMR.F8S2. This instruction is unpredicated. It has encodings from 2 classes: [BF1CVTLT](#iclass_bf1cvtlt) and [BF2CVTLT](#iclass_bf2cvtlt)

#### Decode

if ((!IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME2)) \|\| !IsFeatureImplemented(FEAT_FP8)) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 16; let n : integer = UInt(Zn); let d : integer = UInt(Zd); let issrc2 : boolean = TRUE;

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); if IsFeatureImplemented(FEAT_SME2) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand : bits(VL) = Z{}(n); var result : bits(VL); for e = 0 to elements-1 do let element : bits(esize DIV 2) = operand[(2\*e + 1)\*:(esize DIV 2)]; result[e\*:esize] = [FP8ConvertBF](shared_pseudocode.html#func_FP8ConvertBF_4)(element, issrc2, FPCR(), FPMR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
