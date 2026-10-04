## BFCVTN, BFCVTN2

Single-precision convert to BFloat16 (vector) This instruction reads each single-precision element in the SIMD&FP source vector, converts each value to BFloat16 format, and writes the results in the lower or upper half of the SIMD&FP destination vector. The result elements are half the width of the source elements. The BFCVTN instruction writes the half-width results to the lower half of the 128-bit SIMD&FP destination vector and clears the upper half to zero. The BFCVTN2 instruction writes the results to the upper half of the 128-bit SIMD&FP destination vector without affecting the lower half.

#### Decode

if !IsFeatureImplemented(FEAT_BF16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let part : integer = UInt(Q); let elements : integer = 64 DIV 16;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(128) = V{}(n); var result : bits(64); for e = 0 to elements-1 do result[e\*:16] = [FPConvertBF](shared_pseudocode.html#func_FPConvertBF_2)(operand[e\*:32], FPCR()); end; [Vpart](shared_pseudocode.html#accessor_Vpart_3){64}(d, part) = result;
