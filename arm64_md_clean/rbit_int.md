## RBIT

Reverse bits This instruction reverses the bit order in a register.

#### Decode

let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let datasize : integer{} = 32 << UInt(sf);

### Operation

let operand : bits(datasize) = X{}(n); var result : bits(datasize); for i = 0 to datasize-1 do result[(datasize-1)-i] = operand[i]; end; [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result;
