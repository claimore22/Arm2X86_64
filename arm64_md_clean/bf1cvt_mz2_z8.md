## BF1CVT, BF2CVT

Multi-vector 8-bit floating-point convert to BFloat16 This instruction converts each 8-bit floating-point element of the source vector to BFloat16 while downscaling the value, and places the results in the corresponding 16-bit elements of the destination vectors. BF1CVT scales the values by 2<sup>-UInt(FPMR.LSCALE[5:0])</sup>. BF2CVT scales the values by 2<sup>-UInt(FPMR.LSCALE2[5:0])</sup>. The 8-bit floating-point encoding format for BF1CVT is selected by FPMR.F8S1. The 8-bit floating-point encoding format for BF2CVT is selected by FPMR.F8S2. This instruction is unpredicated. It has encodings from 2 classes: [BF1CVT](#iclass_bf1cvt) and [BF2CVT](#iclass_bf2cvt)

#### Decode

if !IsFeatureImplemented(FEAT_SME2) \|\| !IsFeatureImplemented(FEAT_FP8) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let d : integer = UInt(Zd:: '0'); let issrc2 : boolean = TRUE;

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); [CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 8; let operand : bits(VL) = Z{}(n); var result : bits(2\*VL); for e = 0 to elements-1 do let element : bits(8) = operand[e\*:8]; result[e\*:16] = [FP8ConvertBF](shared_pseudocode.html#func_FP8ConvertBF_4)(element, issrc2, FPCR(), FPMR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+0) = result[VL-1:0]; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+1) = result[2\*VL-1:VL];
