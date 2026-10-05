## UDIV (quotient)

Unsigned divide This instruction divides the first unsigned source register value by the second unsigned source register value, and writes the result to the destination register. Dividing by zero writes the value zero to the destination register. The condition flags are not affected.

#### Decode

let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 32 << UInt(sf);

### Operation

let operand1 : bits(datasize) = X{}(n); let operand2 : bits(datasize) = X{}(m); let dividend : integer = UInt(operand1); let divisor : integer = UInt(operand2); var result : integer; if divisor == 0 then result = 0; else result = dividend DIVRM divisor; end; [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result[datasize-1:0];
