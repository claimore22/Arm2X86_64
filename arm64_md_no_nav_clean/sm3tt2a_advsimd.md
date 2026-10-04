## SM3TT2A

SM3TT2A This instruction takes three 128-bit vectors from three source SIMD&FP register and a 2-bit immediate index value, and returns a 128-bit result in the destination SIMD&FP register. It performs a three-way exclusive-OR of the three 32-bit fields held in the upper three elements of the first source vector, and adds the resulting 32-bit value and the following three other 32-bit values: - The bottom 32-bit element of the first source vector, Vd, that was used for the three-way exclusive-OR. - The 32-bit element held in the top 32 bits of the second source vector, Vn. - A 32-bit element indexed out of the third source vector, Vm. A three-way exclusive-OR is performed of the result of this addition, the result of the addition rotated left by 9, and the result of the addition rotated left by 17. The result of this exclusive-OR is returned as the top element of the returned result. The other elements of this result are taken from elements of the first source vector, with the element returned in bits<63:32> being rotated left by 19.

#### Decode

if !IsFeatureImplemented(FEAT_SM3) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let i : integer = UInt(imm2);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let Vm : bits(128) = V{}(m); let Vn : bits(128) = V{}(n); let Vd : bits(128) = V{}(d); var Wj : bits(32); var result : bits(128); var TT2 : bits(32); Wj = Vm[i\*:32]; TT2 = Vd[63:32] XOR (Vd[127:96] XOR Vd[95:64]); TT2 = (TT2 + Vd[31:0] + Vn[127:96] + Wj)[31:0]; result[31:0] = Vd[63:32]; result[63:32] = ROL(Vd[95:64], 19); result[95:64] = Vd[127:96]; result[127:96] = TT2 XOR ROL(TT2, 9) XOR ROL(TT2, 17); [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;
