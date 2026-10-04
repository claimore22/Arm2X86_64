## SM4EKEY

SM4 key This instruction takes an input as a 128-bit vector from the first source SIMD&FP register and a 128-bit constant from the second SIMD&FP register. It derives four iterations of the output key, in accordance with the SM4 standard, returning the 128-bit result to the destination SIMD&FP register.

#### Decode

if !IsFeatureImplemented(FEAT_SM4) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let Vm : bits(128) = V{}(m); var intval : bits(32); var const : bits(32); var roundresult : bits(128); roundresult = [V](shared_pseudocode.html#accessor_V_2){128}(n); for index = 0 to 3 do const = Vm[index\*:32]; intval = roundresult[127:96] XOR roundresult[95:64] XOR roundresult[63:32] XOR const; for i = 0 to 3 do intval[i\*:8] = [Sbox](shared_pseudocode.html#func_Sbox_1)(intval[i\*:8]); end; intval = intval XOR ROL(intval, 13) XOR ROL(intval, 23); intval = intval XOR roundresult[31:0]; roundresult[31:0] = roundresult[63:32]; roundresult[63:32] = roundresult[95:64]; roundresult[95:64] = roundresult[127:96]; roundresult[127:96] = intval; end; [V](shared_pseudocode.html#accessor_V_2){128}(d) = roundresult;
