## FCVTN, FCVTN2 (single-precision to 8-bit floating-point)

Single-precision convert to 8-bit floating-point (vector) This instruction converts each single-precision element of the two source vectors to 8-bit floating-point while scaling the value by 2<sup>SInt(FPMR.NSCALE)</sup>, and places the in-order results in the 8-bit elements of the lower or upper half of the destination vector. FCVTN writes the results to the lower half of the destination vector and clears the upper half. FCVTN2 writes the results to the upper half of the destination vector without affecting the other bits of the vector. The 8-bit floating-point encoding format is selected by FPMR.F8D.

#### Decode

if !IsFeatureImplemented(FEAT_FP8) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let part : integer = UInt(Q); let elements : integer = 128 DIV 32;

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); [AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(128) = V{}(n); let operand2 : bits(128) = V{}(m); var result : bits(64); for e = 0 to elements-1 do result[(0\*elements + e)\*:8] = [FPConvertFP8](shared_pseudocode.html#func_FPConvertFP8_5){8, 32}(operand1[e\*:32], FPCR(), FPMR()); result[(1\*elements + e)\*:8] = [FPConvertFP8](shared_pseudocode.html#func_FPConvertFP8_5){8, 32}(operand2[e\*:32], FPCR(), FPMR()); end; [Vpart](shared_pseudocode.html#accessor_Vpart_3){64}(d, part) = result;
