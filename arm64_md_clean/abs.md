## ABS

Absolute value This instruction calculates the absolute value of the signed integer value in the source register, and writes the result to the destination register.

#### Decode

if !IsFeatureImplemented(FEAT_CSSC) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let datasize : integer{} = 32 << UInt(sf); let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn);

### Operation

let operand1 : bits(datasize) = X{}(n); let result : integer = Abs(SInt(operand1)); [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result[datasize-1:0];
