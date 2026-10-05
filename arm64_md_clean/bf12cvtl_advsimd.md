## BF1CVTL, BF1CVTL2, BF2CVTL, BF2CVTL2

8-bit floating-point convert to BFloat16 (vector) This instruction converts each 8-bit floating-point element from the lower or upper half of the source vector to BFloat16 while downscaling the value, and places the results in the 16-bit elements of the destination vector. BF1CVTL and BF2CVTL convert the elements from the lower half of the source vector while scaling the values by 2<sup>-UInt(FPMR.LSCALE[5:0])</sup> and 2<sup>-UInt(FPMR.LSCALE2[5:0])</sup>, respectively. BF1CVTL2 and BF2CVTL2 convert the elements from the upper half of the source vector while scaling the values by 2<sup>-UInt(FPMR.LSCALE[5:0])</sup> and 2<sup>-UInt(FPMR.LSCALE2[5:0])</sup>, respectively. The 8-bit floating-point encoding format for BF1CVTL and BF1CVTL2 is selected by FPMR.F8S1. The 8-bit floating-point encoding format for BF2CVTL and BF2CVTL2 is selected by FPMR.F8S2.

#### Decode

if !IsFeatureImplemented(FEAT_FP8) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let part : integer = UInt(Q); let elements : integer = 64 DIV 8; let issrc2 : boolean = size == '11';

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); [AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(64) = Vpart{}(n, part); var result : bits(128); for e = 0 to elements-1 do result[e\*:16] = [FP8ConvertBF](shared_pseudocode.html#func_FP8ConvertBF_4)(operand[e\*:8], issrc2, FPCR(), FPMR()); end; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;
