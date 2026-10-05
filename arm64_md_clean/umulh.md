## UMULH

Unsigned multiply high This instruction multiplies two 64-bit register values, and writes bits[127:64] of the 128-bit result to the 64-bit destination register.

#### Decode

let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm);

### Operation

let operand1 : bits(64) = X{}(n); let operand2 : bits(64) = X{}(m); let result : integer = UInt(operand1) \* UInt(operand2); [X](shared_pseudocode.html#accessor_X_2){64}(d) = result[127:64];
