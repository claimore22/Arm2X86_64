## SM3SS1

SM3SS1 This instruction rotates the top 32 bits of the 128-bit vector in the first source SIMD&FP register by 12, and adds that 32-bit value to the two other 32-bit values held in the top 32 bits of each of the 128-bit vectors in the second and third source SIMD&FP registers, rotating this result left by 7 and writing the final result into the top 32 bits of the vector in the destination SIMD&FP register, with the bottom 96 bits of the vector being written to 0.

#### Decode

if !IsFeatureImplemented(FEAT_SM3) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let a : integer{} = UInt(Ra);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let Vm : bits(128) = V{}(m); let Vn : bits(128) = V{}(n); let Va : bits(128) = V{}(a); var result : bits(128); result[127:96] = ROL((ROL(Vn[127:96], 12) + Vm[127:96] + Va[127:96]), 7); result[95:0] = Zeros{96}; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;
