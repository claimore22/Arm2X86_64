## SHA1SU0

SHA1 schedule update 0 SHA1 schedule update 0.

#### Decode

if !IsFeatureImplemented(FEAT_SHA1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(128) = V{}(d); let operand2 : bits(128) = V{}(n); let operand3 : bits(128) = V{}(m); var result : bits(128) = operand2[63:0] :: operand1[127:64]; result = result XOR operand1 XOR operand3; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;
