## ADDSVL

Add multiple of Streaming SVE vector register size to scalar register This instruction adds the Streaming SVE vector register size in bytes multiplied by an immediate in the range -32 to 31 to the 64-bit source general-purpose register or current stack pointer, and places the result in the 64-bit destination general-purpose register or current stack pointer. This instruction does not require the PE to be in Streaming SVE mode.

#### Decode

if !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Rn); let d : integer = UInt(Rd); let imm : integer = SInt(imm6);

### Operation

[CheckSMEEnabled](shared_pseudocode.html#func_CheckSMEEnabled_0)(); let SVL : integer = [CurrentSVL](shared_pseudocode.html#func_CurrentSVL_0)(); let len : integer = imm \* (SVL DIV 8); let operand1 : bits(64) = if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() else [X](shared_pseudocode.html#accessor_X_2){64}(n); let result : bits(64) = operand1 + len; if d == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = result; else [X](shared_pseudocode.html#accessor_X_2){64}(d) = result; end;
