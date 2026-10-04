## FCVTN (half-precision to 8-bit floating-point)

Half-precision convert to 8-bit floating-point (vector) This instruction converts half-precision elements of the two source vectors to 8-bit floating-point while scaling the values by 2<sup>SInt(FPMR.NSCALE[4:0])</sup>, and places the in-order results in the 8-bit elements of the destination vector. The 8-bit floating-point encoding format is selected by FPMR.F8D.

#### Decode

if !IsFeatureImplemented(FEAT_FP8) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = if Q == '1' then 128 else 64; let elements : integer = datasize DIV 16;

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); [AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(datasize) = V{}(n); let operand2 : bits(datasize) = V{}(m); var result : bits(datasize); for e = 0 to elements-1 do result[(0\*elements + e)\*:8] = [FPConvertFP8](shared_pseudocode.html#func_FPConvertFP8_5){8, 16}(operand1[e\*:16], FPCR(), FPMR()); result[(1\*elements + e)\*:8] = [FPConvertFP8](shared_pseudocode.html#func_FPConvertFP8_5){8, 16}(operand2[e\*:16], FPCR(), FPMR()); end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;
