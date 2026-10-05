## SDIV (quotient)

Signed divide This instruction divides the first signed source register value by the second signed source register value, and writes the result to the destination register. Dividing by zero writes the value zero to the destination register. The condition flags are not affected.

#### Decode

let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 32 << UInt(sf);

### Operation

let operand1 : bits(datasize) = X{}(n); let operand2 : bits(datasize) = X{}(m); let dividend : integer = SInt(operand1); let divisor : integer = SInt(operand2); var result : integer; if divisor == 0 then result = 0; elsif (dividend < 0) == (divisor < 0) then result = Abs(dividend) DIVRM Abs(divisor); // same signs - positive result else result = -(Abs(dividend) DIVRM Abs(divisor)); // different signs - negative result end; [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result[datasize-1:0];
