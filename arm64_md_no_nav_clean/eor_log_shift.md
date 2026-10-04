## EOR (shifted register)

Bitwise exclusive-OR (shifted register) This instruction performs a bitwise exclusive-OR between a register value and an optionally-shifted register value, and writes the result to the destination register.

#### Decode

if sf == '0' && imm6[5] == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 32 << UInt(sf); let shift_type : [ShiftType](shared_pseudocode.html#type_ShiftType) = [DecodeShift](shared_pseudocode.html#func_DecodeShift_1)(shift); let shift_amount : integer = UInt(imm6);

### Operation

let operand1 : bits(datasize) = X{}(n); let operand2 : bits(datasize) = ShiftReg{}(m, shift_type, shift_amount); [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = operand1 XOR operand2;
