## SMSUBL

Signed multiply-subtract long This instruction multiplies two 32-bit register values, subtracts the product from a 64-bit register value, and writes the result to the 64-bit destination register. This instruction is used by the alias [SMNEGL](smnegl_smsubl.html "Signed multiply-negate long").

#### Decode

let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let a : integer{} = UInt(Ra);

### Operation

let operand1 : bits(32) = X{}(n); let operand2 : bits(32) = X{}(m); let operand3 : bits(64) = X{}(a); let result : integer = SInt(operand3) - (SInt(operand1) \* SInt(operand2)); [X](shared_pseudocode.html#accessor_X_2){64}(d) = result[63:0];
