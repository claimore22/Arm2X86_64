## CTZ

Count trailing zeros This instruction determines the number of consecutive binary zero bits, starting from the least significant bit in the source register, and places the result in the destination register.

#### Decode

if !IsFeatureImplemented(FEAT_CSSC) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let datasize : integer{} = 32 << UInt(sf); let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn);

### Operation

let operand1 : bits(datasize) = X{}(n); let result : integer = CountLeadingZeroBits([BitReverse](shared_pseudocode.html#func_BitReverse_2){datasize}(operand1)); [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result[datasize-1:0];
