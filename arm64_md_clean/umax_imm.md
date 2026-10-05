## UMAX (immediate)

Unsigned maximum (immediate) This instruction determines the unsigned maximum of the source register value and immediate, and writes the result to the destination register.

#### Decode

if !IsFeatureImplemented(FEAT_CSSC) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let datasize : integer{} = 32 << UInt(sf); let imm : integer = UInt(imm8);

### Operation

let operand1 : bits(datasize) = X{}(n); let result : integer = Max(UInt(operand1), imm); [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result[datasize-1:0];
