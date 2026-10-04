## XAR

Exclusive-OR and rotate This instruction performs a bitwise exclusive-OR between the 128-bit vector in the first source SIMD&FP register and the 128-bit vector in the second source SIMD&FP register, rotates each 64-bit element of the resulting 128-bit vector right by the value specified by a 6-bit immediate value, and writes the result to the destination SIMD&FP register.

#### Decode

if !IsFeatureImplemented(FEAT_SHA3) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let Vm : bits(128) = V{}(m); let Vn : bits(128) = V{}(n); let tmp : bits(128) = Vn XOR Vm; [V](shared_pseudocode.html#accessor_V_2){128}(d) = ROR(tmp[127:64], UInt(imm6))::ROR(tmp[63:0], UInt(imm6));
