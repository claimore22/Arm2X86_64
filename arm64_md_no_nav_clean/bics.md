## BICS (shifted register)

Bitwise bit clear (shifted register), setting flags This instruction performs a bitwise AND between a register value and the complement of an optionally-shifted register value, and writes the result to the destination register. It updates the condition flags based on the result.

#### Decode

if sf == '0' && imm6[5] == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 32 << UInt(sf); let shift_type : [ShiftType](shared_pseudocode.html#type_ShiftType) = [DecodeShift](shared_pseudocode.html#func_DecodeShift_1)(shift); let shift_amount : integer = UInt(imm6);

### Operation

let operand1 : bits(datasize) = X{}(n); let operand2 : bits(datasize) = ShiftReg{}(m, shift_type, shift_amount); let result : bits(datasize) = operand1 AND NOT(operand2); [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result; PSTATE.[N,Z,C,V] = result[datasize-1]::[IsZeroBit](shared_pseudocode.html#func_IsZeroBit_2){datasize}(result)::'00';
