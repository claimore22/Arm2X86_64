## MSUB

Multiply-subtract This instruction multiplies two register values, subtracts the product from a third register value, and writes the result to the destination register. This instruction is used by the alias [MNEG](mneg_msub.html "Multiply-negate").

#### Decode

let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let a : integer{} = UInt(Ra); let datasize : integer{} = 32 << UInt(sf);

### Operation

let operand1 : bits(datasize) = X{}(n); let operand2 : bits(datasize) = X{}(m); let operand3 : bits(datasize) = X{}(a); let result : integer = UInt(operand3) - (UInt(operand1) \* UInt(operand2)); [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result[datasize-1:0];
