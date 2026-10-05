## SM3PARTW1

SM3PARTW1 This instruction takes three 128-bit vectors from the three source SIMD&FP registers and returns a 128-bit result in the destination SIMD&FP register. The result is obtained by a three-way exclusive-OR of the elements within the input vectors with some fixed rotations, see the Operation pseudocode for more information.

#### Decode

if !IsFeatureImplemented(FEAT_SM3) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let Vm : bits(128) = V{}(m); let Vn : bits(128) = V{}(n); let Vd : bits(128) = V{}(d); var result : bits(128); result[95:0] = (Vd XOR Vn)[95:0] XOR (ROL(Vm[127:96], 15)::ROL(Vm[95:64], 15)::ROL(Vm[63:32], 15)); for i = 0 to 3 do if i == 3 then result[127:96] = (Vd XOR Vn)[127:96] XOR (ROL(result[31:0], 15)); end; result[(32\*i)+31:(32\*i)] = (result[(32\*i)+31:(32\*i)] XOR ROL(result[(32\*i)+31:(32\*i)], 15) XOR ROL(result[(32\*i)+31:(32\*i)], 23)); end; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;
