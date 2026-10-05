## ADD (immediate)

Add immediate (unpredicated) This instruction adds an unsigned immediate to each element of the source vector, and destructively places the results in the corresponding elements of the source vector. This instruction is unpredicated. The immediate is an unsigned value in the range 0 to 255, and for element widths of 16 bits or higher it may also be a positive multiple of 256 in the range 256 to 65280. The immediate is encoded in 8 bits with an optional left shift by 8. The preferred disassembly when the shift option is specified is "#<imm>, LSL \#8". However an assembler and disassembler may also allow use of the shifted 16-bit value unless the immediate is 0 and the shift amount is 8, which must be unambiguously described as "#0, LSL \#8".

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size::sh == '001' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let dn : integer = UInt(Zdn); var imm : integer = UInt(imm8); if sh == '1' then imm = imm << 8; end;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(dn); var result : bits(VL); for e = 0 to elements-1 do let element1 : bits(esize) = operand1[e\*:esize]; result[e\*:esize] = element1 + imm; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;
