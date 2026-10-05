## SM4E

SM4 encode This instruction takes input data as a 128-bit vector from the first source SIMD&FP register, and four iterations of the round key held as the elements of the 128-bit vector in the second source SIMD&FP register. It encrypts the data by four rounds, in accordance with the SM4 standard, returning the 128-bit result to the destination SIMD&FP register.

#### Decode

if !IsFeatureImplemented(FEAT_SM4) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let Vn : bits(128) = V{}(n); var intval : bits(32); var roundresult : bits(128); var roundkey : bits(32); roundresult = [V](shared_pseudocode.html#accessor_V_2){128}(d); for index = 0 to 3 do roundkey = Vn[index\*:32]; intval = roundresult[127:96] XOR roundresult[95:64] XOR roundresult[63:32] XOR roundkey; for i = 0 to 3 do intval[i\*:8] = [Sbox](shared_pseudocode.html#func_Sbox_1)(intval[i\*:8]); end; intval = intval XOR ROL(intval, 2) XOR ROL(intval, 10) XOR ROL(intval, 18) XOR ROL(intval, 24); intval = intval XOR roundresult[31:0]; roundresult[31:0] = roundresult[63:32]; roundresult[63:32] = roundresult[95:64]; roundresult[95:64] = roundresult[127:96]; roundresult[127:96] = intval; end; [V](shared_pseudocode.html#accessor_V_2){128}(d) = roundresult;
