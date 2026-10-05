## ADC

Add with carry This instruction adds two register values and the Carry flag value, and writes the result to the destination register.

#### Decode

let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 32 << UInt(sf);

### Operation

let operand1 : bits(datasize) = X{}(n); let operand2 : bits(datasize) = X{}(m); var result : bits(datasize); (result, -) = [AddWithCarry](shared_pseudocode.html#func_AddWithCarry_4){datasize}(operand1, operand2, [PSTATE](shared_pseudocode.html#global_PSTATE).C); [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result;
