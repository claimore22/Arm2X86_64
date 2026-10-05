## SBCS

Subtract with carry, setting flags This instruction subtracts a register value and the value of NOT (Carry flag) from a register value, and writes the result to the destination register. It updates the condition flags based on the result. This instruction is used by the alias [NGCS](ngcs_sbcs.html "Negate with carry, setting flags").

#### Decode

let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 32 << UInt(sf);

### Operation

let operand1 : bits(datasize) = X{}(n); let operand2 : bits(datasize) = NOT([X](shared_pseudocode.html#accessor_X_2){datasize}(m)); var result : bits(datasize); var nzcv : bits(4); (result, nzcv) = [AddWithCarry](shared_pseudocode.html#func_AddWithCarry_4){datasize}(operand1, operand2, [PSTATE](shared_pseudocode.html#global_PSTATE).C); [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result; PSTATE.[N,Z,C,V] = nzcv;
