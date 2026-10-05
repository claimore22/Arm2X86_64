## EXTR

Extract register This instruction extracts a register from a pair of registers. This instruction is used by the alias [ROR (immediate)](ror_extr.html "Rotate right (immediate)").

#### Decode

if N != sf then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if sf == '0' && imms[5] == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 32 << UInt(sf); let lsb : integer{} = UInt(imms);

### Operation

var result : bits(datasize); let operand1 : bits(datasize) = X{}(n); let operand2 : bits(datasize) = X{}(m); let concat : bits(2\*datasize) = operand1::operand2; result = concat[(lsb+datasize)-1:lsb]; [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result;
