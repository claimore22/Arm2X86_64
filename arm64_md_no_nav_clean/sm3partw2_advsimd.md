## SM3PARTW2

SM3PARTW2 This instruction takes three 128-bit vectors from three source SIMD&FP registers and returns a 128-bit result in the destination SIMD&FP register. The result is obtained by a three-way exclusive-OR of the elements within the input vectors with some fixed rotations, see the Operation pseudocode for more information.

#### Decode

if !IsFeatureImplemented(FEAT_SM3) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let Vm : bits(128) = V{}(m); let Vn : bits(128) = V{}(n); let Vd : bits(128) = V{}(d); var result : bits(128); var tmp : bits(128); var tmp2 : bits(32); tmp[127:0] = (Vn XOR (ROL(Vm[127:96], 7) :: ROL(Vm[95:64], 7) :: ROL(Vm[63:32], 7) :: ROL(Vm[31:0], 7))); result[127:0] = Vd[127:0] XOR tmp[127:0]; tmp2 = ROL(tmp[31:0], 15); tmp2 = tmp2 XOR ROL(tmp2, 15) XOR ROL(tmp2, 23); result[127:96] = result[127:96] XOR tmp2; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;
