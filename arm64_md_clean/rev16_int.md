## REV16

Reverse bytes in 16-bit halfwords This instruction reverses the byte order in each 16-bit halfword of a register.

#### Decode

if opc == '11' && sf == '0' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let datasize : integer{} = 32 << UInt(sf); let container_size : integer{} = 8 << UInt(opc);

### Operation

let operand : bits(datasize) = X{}(n); var result : bits(datasize); let containers : integer = datasize DIV container_size; for c = 0 to containers-1 do let container : bits(container_size) = operand[c\*:container_size]; result[c\*:container_size] = [Reverse](shared_pseudocode.html#func_Reverse_3){container_size}(container, 8); end; [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result;
