## LSRV

Logical shift right variable This instruction shifts a register value right by a variable number of bits, shifting in zeros, and writes the result to the destination register. The value of the second source register modulo the register size in bits gives the number of bits by which the first source register is right-shifted. This instruction is used by the alias [LSR (register)](lsr_lsrv.html "Logical shift right (register)").

#### Decode

let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 32 << UInt(sf); let shift_type : [ShiftType](shared_pseudocode.html#type_ShiftType) = [DecodeShift](shared_pseudocode.html#func_DecodeShift_1)(op2);

### Operation

let operand2 : bits(datasize) = X{}(m); [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = [ShiftReg](shared_pseudocode.html#func_ShiftReg_4){datasize}(n, shift_type, UInt(operand2) MOD datasize);
