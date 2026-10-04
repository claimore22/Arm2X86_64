## SHA1SU1

SHA1 schedule update 1 SHA1 schedule update 1.

#### Decode

if !IsFeatureImplemented(FEAT_SHA1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(128) = V{}(d); let operand2 : bits(128) = V{}(n); let T : bits(128) = operand1 XOR LSR(operand2, 32); var result : bits(128); result[31:0] = ROL(T[31:0], 1); result[63:32] = ROL(T[63:32], 1); result[95:64] = ROL(T[95:64], 1); result[127:96] = ROL(T[127:96], 1) XOR ROL(T[31:0], 2); [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;
