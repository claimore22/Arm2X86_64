## SHA256SU1

SHA-256 schedule update 1 SHA-256 schedule update 1.

#### Decode

if !IsFeatureImplemented(FEAT_SHA256) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(128) = V{}(d); let operand2 : bits(128) = V{}(n); let operand3 : bits(128) = V{}(m); let T0 : bits(128) = operand3[31:0] :: operand2[127:32]; var T1 : bits(64); var elt : bits(32); var result : bits(128); T1 = operand3[127:64]; for e = 0 to 1 do elt = T1[e\*:32]; elt = ROR(elt, 17) XOR ROR(elt, 19) XOR LSR(elt, 10); elt = elt + operand1[e\*:32] + T0[e\*:32]; result[e\*:32] = elt; end; T1 = result[63:0]; for e = 2 to 3 do elt = T1[(e - 2)\*:32]; elt = ROR(elt, 17) XOR ROR(elt, 19) XOR LSR(elt, 10); elt = elt + operand1[e\*:32] + T0[e\*:32]; result[e\*:32] = elt; end; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;
