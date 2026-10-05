## FDUP

Broadcast floating-point immediate to vector elements (unpredicated) This instruction unconditionally broadcasts the floating-point immediate into each element of the destination vector. This instruction is unpredicated. This instruction is used by the alias [FMOV (immediate, unpredicated)](fmov_fdup_z_i.html "Move floating-point immediate to vector elements (unpredicated)").

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let d : integer = UInt(Zd); let imm : bits(esize) = VFPExpandImm{}(imm8);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; var result : bits(VL); for e = 0 to elements-1 do result[e\*:esize] = imm; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
