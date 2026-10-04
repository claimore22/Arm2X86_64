## SUBS (shifted register)

Subtract optionally-shifted register, setting flags This instruction subtracts an optionally-shifted register value from a register value, and writes the result to the destination register. It updates the condition flags based on the result. This instruction is used by the aliases [CMP (shifted register)](cmp_subs_addsub_shift.html "Compare (shifted register)"), and [NEGS](negs_subs_addsub_shift.html "Negate, setting flags").

#### Decode

if shift == '11' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if sf == '0' && imm6[5] == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 32 << UInt(sf); let shift_type : [ShiftType](shared_pseudocode.html#type_ShiftType) = [DecodeShift](shared_pseudocode.html#func_DecodeShift_1)(shift); let shift_amount : integer = UInt(imm6);

### Operation

let operand1 : bits(datasize) = X{}(n); let operand2 : bits(datasize) = NOT([ShiftReg](shared_pseudocode.html#func_ShiftReg_4){datasize}(m, shift_type, shift_amount)); var result : bits(datasize); var nzcv : bits(4); (result, nzcv) = [AddWithCarry](shared_pseudocode.html#func_AddWithCarry_4){datasize}(operand1, operand2, '1'); [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result; PSTATE.[N,Z,C,V] = nzcv;
