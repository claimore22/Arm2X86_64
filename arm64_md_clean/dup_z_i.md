## DUP (immediate)

Broadcast signed immediate to vector elements (unpredicated) This instruction unconditionally broadcasts the signed integer immediate into each element of the destination vector. This instruction is unpredicated. The immediate operand is a signed value in the range -128 to +127, and for element widths of 16 bits or higher it may also be a signed multiple of 256 in the range -32768 to +32512 (excluding 0). The immediate is encoded in 8 bits with an optional left shift by 8. The preferred disassembly when the shift option is specified is "#<imm>, LSL \#8". However an assembler and disassembler may also allow use of the shifted 16-bit value unless the immediate is 0 and the shift amount is 8, which must be unambiguously described as "#0, LSL \#8". This instruction is used by the alias [MOV (immediate, unpredicated)](mov_dup_z_i.html "Move signed immediate to vector elements (unpredicated)"). This instruction is used by the pseudo-instruction [FMOV (zero, unpredicated)](fmov_dup_z_i.html "Move floating-point +0.0 to vector elements (unpredicated)").

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size::sh == '001' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let d : integer = UInt(Zd); var imm : integer = SInt(imm8); if sh == '1' then imm = imm << 8; end;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let result : bits(VL) = Replicate{}(imm[esize-1:0]); [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
