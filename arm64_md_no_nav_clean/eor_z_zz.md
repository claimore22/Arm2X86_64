## EOR (vectors, unpredicated)

Bitwise exclusive-OR (unpredicated) This instruction performs a bitwise exclusive-OR between the elements of the first source vector and the corresponding elements of the second source vector, and places the results in the corresponding elements of the destination vector. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = operand1 XOR operand2;
