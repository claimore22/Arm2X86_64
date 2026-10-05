## DUPM

Broadcast logical bitmask immediate to vector (unpredicated) This instruction unconditionally broadcasts the logical bitmask immediate into each element of the destination vector. This instruction is unpredicated. The immediate is a 64-bit value consisting of a single run of ones or zeros repeating every 2, 4, 8, 16, 32 or 64 bits. This instruction is used by the alias [MOV](mov_dupm_z_i.html "Move logical bitmask immediate to vector (unpredicated)").

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 64; let d : integer = UInt(Zd); var imm : bits(esize); (imm, -) = [DecodeBitMasks](shared_pseudocode.html#func_DecodeBitMasks_5){esize}(imm13[12], imm13[5:0], imm13[11:6], TRUE);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let result : bits(VL) = Replicate{}(imm); [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
