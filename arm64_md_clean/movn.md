## MOVN

Move wide with NOT This instruction moves the inverse of an optionally-shifted 16-bit immediate value to a register. This instruction is used by the alias [MOV (inverted wide immediate)](mov_movn.html "Move inverted wide immediate value").

#### Decode

if sf == '0' && hw[1] == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = UInt(Rd); let datasize : integer{} = 32 << UInt(sf); let imm : bits(16) = imm16; let pos : integer{} = UInt(hw) << 4;

### Operation

var result : bits(datasize) = Zeros{}; result[pos+15:pos] = imm; [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = NOT(result);
