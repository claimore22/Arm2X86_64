## REV (vector)

Reverse all elements in a vector (unpredicated) This instruction reads all elements in the source vector, reverses the order of all the elements, and places the result in the destination vector. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn); let d : integer = UInt(Zd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let operand : bits(VL) = Z{}(n); let result : bits(VL) = Reverse{}(operand, esize); [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
