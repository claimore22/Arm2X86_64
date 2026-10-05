## DUP (scalar)

Broadcast general-purpose register to vector elements (unpredicated) This instruction unconditionally broadcasts the general-purpose scalar source register into each element of the destination vector. This instruction is unpredicated. This instruction is used by the alias [MOV (scalar, unpredicated)](mov_dup_z_r.html "Move general-purpose register to vector elements (unpredicated)").

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Rn); let d : integer = UInt(Zd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; var operand : bits(64); if n == 31 then operand = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else operand = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; var result : bits(VL); for e = 0 to elements-1 do result[e\*:esize] = operand[esize-1:0]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
