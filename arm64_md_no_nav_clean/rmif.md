## RMIF

Rotate, mask insert flags This instruction performs a rotation right of a value held in a general-purpose register by an immediate value, and then inserts a selection of the bottom four bits of the result of the rotation into the PSTATE flags, under the control of a second immediate mask.

#### Decode

if !IsFeatureImplemented(FEAT_FlagM) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let imm : integer{} = UInt(imm6); let flagmask : bits(4) = mask; let n : integer{} = UInt(Rn);

### Operation

let reg : bits(64) = X{}(n); let flags : bits(4) = (reg::reg)[imm+3:imm]; if flagmask[3] == '1' then [PSTATE](shared_pseudocode.html#global_PSTATE).N = flags[3]; end; if flagmask[2] == '1' then [PSTATE](shared_pseudocode.html#global_PSTATE).Z = flags[2]; end; if flagmask[1] == '1' then [PSTATE](shared_pseudocode.html#global_PSTATE).C = flags[1]; end; if flagmask[0] == '1' then [PSTATE](shared_pseudocode.html#global_PSTATE).V = flags[0]; end;
