## F1CVTL, F2CVTL

Multi-vector 8-bit floating-point convert to deinterleaved half-precision This instruction converts each 8-bit floating-point element of the source vector to half-precision while downscaling the value, and places the two-way deinterleaved results in the corresponding 16-bit elements of the destination vectors. F1CVTL scales the values by 2<sup>-UInt(FPMR.LSCALE[3:0])</sup>. F2CVTL scales the values by 2<sup>-UInt(FPMR.LSCALE2[3:0])</sup>. The 8-bit floating-point encoding format for F1CVTL is selected by FPMR.F8S1. The 8-bit floating-point encoding format for F2CVTL is selected by FPMR.F8S2. This instruction is unpredicated. It has encodings from 2 classes: [F1CVTL](#iclass_f1cvtl) and [F2CVTL](#iclass_f2cvtl)

#### Decode

if !IsFeatureImplemented(FEAT_SME2) \|\| !IsFeatureImplemented(FEAT_FP8) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let d : integer = UInt(Zd:: '0'); let issrc2 : boolean = TRUE;

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); [CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let pairs : integer = VL DIV 16; let operand : bits(VL) = Z{}(n); var result1 : bits(VL); var result2 : bits(VL); for p = 0 to pairs-1 do let element1 : bits(8) = operand[(2\*p + 0)\*:8]; let element2 : bits(8) = operand[(2\*p + 1)\*:8]; result1[p\*:16] = [FP8ConvertFP](shared_pseudocode.html#func_FP8ConvertFP_4)(element1, issrc2, FPCR(), FPMR()); result2[p\*:16] = [FP8ConvertFP](shared_pseudocode.html#func_FP8ConvertFP_4)(element2, issrc2, FPCR(), FPMR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+0) = result1; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+1) = result2;
