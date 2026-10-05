## SHA512H

SHA-512 hash update part 1 This instruction takes the values from the three 128-bit source SIMD&FP registers and produces a 128-bit output value that combines the sigma1 and chi functions of two iterations of the SHA-512 calculation. It returns this value to the destination SIMD&FP register.

#### Decode

if !IsFeatureImplemented(FEAT_SHA512) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); var Vtmp : bits(128); var MSigma1 : bits(64); var tmp : bits(64); let x : bits(128) = V{}(n); let y : bits(128) = V{}(m); let w : bits(128) = V{}(d); MSigma1 = ROR(y[127:64], 14) XOR ROR(y[127:64], 18) XOR ROR(y[127:64], 41); Vtmp[127:64] = (y[127:64] AND x[63:0]) XOR (NOT(y[127:64]) AND x[127:64]); Vtmp[127:64] = (Vtmp[127:64] + MSigma1 + w[127:64]); tmp = Vtmp[127:64] + y[63:0]; MSigma1 = ROR(tmp, 14) XOR ROR(tmp, 18) XOR ROR(tmp, 41); Vtmp[63:0] = (tmp AND y[127:64]) XOR (NOT(tmp) AND x[63:0]); Vtmp[63:0] = (Vtmp[63:0] + MSigma1 + w[63:0]); [V](shared_pseudocode.html#accessor_V_2){128}(d) = Vtmp;
