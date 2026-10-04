## MOVK

Move wide with keep This instruction moves an optionally-shifted 16-bit immediate value into a register, keeping other bits unchanged.

#### Decode

if sf == '0' && hw[1] == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = UInt(Rd); let datasize : integer{} = 32 << UInt(sf); let imm : bits(16) = imm16; let pos : integer{} = UInt(hw) << 4;

### Operation

var result : bits(datasize) = X{}(d); result[pos+15:pos] = imm; [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result;
