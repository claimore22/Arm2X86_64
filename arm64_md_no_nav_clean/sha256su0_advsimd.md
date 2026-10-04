## SHA256SU0

SHA-256 schedule update 0 SHA-256 schedule update 0.

#### Decode

if !IsFeatureImplemented(FEAT_SHA256) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(128) = V{}(d); let operand2 : bits(128) = V{}(n); let T : bits(128) = operand2[31:0] :: operand1[127:32]; var result : bits(128); var elt : bits(32); for e = 0 to 3 do elt = T[e\*:32]; elt = ROR(elt, 7) XOR ROR(elt, 18) XOR LSR(elt, 3); result[e\*:32] = elt + operand1[e\*:32]; end; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;
