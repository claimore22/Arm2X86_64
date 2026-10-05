## ADDPL

Add multiple of predicate register size to scalar register This instruction adds the current predicate register size in bytes multiplied by an immediate in the range -32 to 31 to the 64-bit source general-purpose register or current stack pointer and places the result in the 64-bit destination general-purpose register or current stack pointer.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Rn); let d : integer = UInt(Rd); let imm : integer = SInt(imm6);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let operand1 : bits(64) = if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() else [X](shared_pseudocode.html#accessor_X_2){64}(n); let result : bits(64) = operand1 + (imm \* (PL DIV 8)); if d == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = result; else [X](shared_pseudocode.html#accessor_X_2){64}(d) = result; end;
