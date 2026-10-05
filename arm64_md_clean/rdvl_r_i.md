## RDVL

Read multiple of vector register size to scalar register This instruction multiplies the current vector register size in bytes by an immediate in the range -32 to 31 and places the result in the 64-bit destination general-purpose register.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = UInt(Rd); let imm : integer = SInt(imm6);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let len : integer = imm \* (VL DIV 8); [X](shared_pseudocode.html#accessor_X_2){64}(d) = len[63:0];
