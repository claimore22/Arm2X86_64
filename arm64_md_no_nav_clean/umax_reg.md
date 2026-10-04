## UMAX (register)

Unsigned maximum (register) This instruction determines the unsigned maximum of the two source register values and writes the result to the destination register.

#### Decode

if !IsFeatureImplemented(FEAT_CSSC) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 32 << UInt(sf);

### Operation

let operand1 : bits(datasize) = X{}(n); let operand2 : bits(datasize) = X{}(m); let result : integer = Max(UInt(operand1), UInt(operand2)); [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result[datasize-1:0];
