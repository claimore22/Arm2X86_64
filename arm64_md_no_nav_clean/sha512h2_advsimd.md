## SHA512H2

SHA-512 hash update part 2 This instruction takes the values from the three 128-bit source SIMD&FP registers and produces a 128-bit output value that combines the sigma0 and majority functions of two iterations of the SHA-512 calculation. It returns this value to the destination SIMD&FP register.

#### Decode

if !IsFeatureImplemented(FEAT_SHA512) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); var Vtmp : bits(128); var NSigma0 : bits(64); let x : bits(128) = V{}(n); let y : bits(128) = V{}(m); let w : bits(128) = V{}(d); NSigma0 = ROR(y[63:0], 28) XOR ROR(y[63:0], 34) XOR ROR(y[63:0], 39); Vtmp[127:64] = [SHAmajority](shared_pseudocode.html#func_SHAmajority_4){64}(x[63:0], y[127:64], y[63:0]); Vtmp[127:64] = (Vtmp[127:64] + NSigma0 + w[127:64]); NSigma0 = ROR(Vtmp[127:64], 28) XOR ROR(Vtmp[127:64], 34) XOR ROR(Vtmp[127:64], 39); Vtmp[63:0] = [SHAmajority](shared_pseudocode.html#func_SHAmajority_4){64}(Vtmp[127:64], y[63:0], y[127:64]); Vtmp[63:0] = (Vtmp[63:0] + NSigma0 + w[63:0]); [V](shared_pseudocode.html#accessor_V_2){128}(d) = Vtmp;
