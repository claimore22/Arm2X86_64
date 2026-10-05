## RDSVL

Read multiple of Streaming SVE vector register size to scalar register This instruction multiplies the Streaming SVE vector register size in bytes by an immediate in the range -32 to 31 and places the result in the 64-bit destination general-purpose register. This instruction does not require the PE to be in Streaming SVE mode.

#### Decode

if !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = UInt(Rd); let imm : integer = SInt(imm6);

### Operation

[CheckSMEEnabled](shared_pseudocode.html#func_CheckSMEEnabled_0)(); let SVL : integer = [CurrentSVL](shared_pseudocode.html#func_CurrentSVL_0)(); let len : integer = imm \* (SVL DIV 8); [X](shared_pseudocode.html#accessor_X_2){64}(d) = len[63:0];
