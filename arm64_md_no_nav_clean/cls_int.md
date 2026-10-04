## CLS

Count leading sign bits This instruction determines the number of leading bits of the source register that have the same value as the most significant bit of the register, and writes the result to the destination register. The result does not include the most significant bit of the source register.

#### Decode

let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let datasize : integer{} = 32 << UInt(sf);

### Operation

let operand1 : bits(datasize) = X{}(n); let result : integer = CountLeadingSignBits(operand1); [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result[datasize-1:0];
