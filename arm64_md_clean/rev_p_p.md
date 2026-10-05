## REV (predicate)

Reverse all elements in a predicate This instruction reads all elements in the source predicate, reverses the order of all the elements, and places the result in the destination predicate. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Pn); let d : integer = UInt(Pd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let operand : bits(PL) = P{}(n); let result : bits(PL) = Reverse{}(operand, esize DIV 8); [P](shared_pseudocode.html#accessor_P_2){PL}(d) = result;
