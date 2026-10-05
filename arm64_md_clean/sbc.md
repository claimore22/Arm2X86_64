## SBC

Subtract with carry This instruction subtracts a register value and the value of NOT (Carry flag) from a register value, and writes the result to the destination register. This instruction is used by the alias [NGC](ngc_sbc.html "Negate with carry").

#### Decode

let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 32 << UInt(sf);

### Operation

let operand1 : bits(datasize) = X{}(n); let operand2 : bits(datasize) = NOT([X](shared_pseudocode.html#accessor_X_2){datasize}(m)); var result : bits(datasize); (result, -) = [AddWithCarry](shared_pseudocode.html#func_AddWithCarry_4){datasize}(operand1, operand2, [PSTATE](shared_pseudocode.html#global_PSTATE).C); [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result;
