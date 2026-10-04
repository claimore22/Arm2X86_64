## SETF8, SETF16

Evaluation of 8-bit or 16-bit flag values This instruction sets the PSTATE.NZV flags based on the value in the specified general-purpose register. SETF8 treats the value as an 8-bit value. SETF16 treats the value as a 16-bit value. The PSTATE.C flag is not affected by these instructions.

#### Decode

if !IsFeatureImplemented(FEAT_FlagM) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let size : integer{} = 8 << UInt(sz); let n : integer = UInt(Rn);

### Operation

let reg : bits(32) = X{}(n); [PSTATE](shared_pseudocode.html#global_PSTATE).N = reg[size-1]; [PSTATE](shared_pseudocode.html#global_PSTATE).Z = if (reg[size-1:0] == Zeros{size}) then '1' else '0'; [PSTATE](shared_pseudocode.html#global_PSTATE).V = reg[size] XOR reg[size-1]; //PSTATE.C unchanged;
