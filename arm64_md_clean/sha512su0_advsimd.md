## SHA512SU0

SHA-512 schedule update 0 This instruction takes the values from the two 128-bit source SIMD&FP registers and produces a 128-bit output value that combines the gamma0 functions of two iterations of the SHA-512 schedule update that are performed after the first 16 iterations within a block. It returns this value to the destination SIMD&FP register.

#### Decode

if !IsFeatureImplemented(FEAT_SHA512) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); var sig0 : bits(64); var Vtmp : bits(128); let x : bits(128) = V{}(n); let w : bits(128) = V{}(d); sig0 = ROR(w[127:64], 1) XOR ROR(w[127:64], 8) XOR ('0000000'::w[127:71]); Vtmp[63:0] = w[63:0] + sig0; sig0 = ROR(x[63:0], 1) XOR ROR(x[63:0], 8) XOR ('0000000'::x[63:7]); Vtmp[127:64] = w[127:64] + sig0; [V](shared_pseudocode.html#accessor_V_2){128}(d) = Vtmp;
