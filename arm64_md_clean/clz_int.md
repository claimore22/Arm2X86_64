## CLZ

Count leading zeros This instruction determines the number of consecutive binary zero bits, starting from the most significant bit in the source register, and places the result in the destination register.

#### Decode

let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let datasize : integer{} = 32 << UInt(sf);

### Operation

let operand1 : bits(datasize) = X{}(n); let result : integer = CountLeadingZeroBits(operand1); [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result[datasize-1:0];
