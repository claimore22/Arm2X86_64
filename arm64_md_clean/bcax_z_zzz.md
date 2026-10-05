## BCAX

Bitwise clear and exclusive-OR This instruction performs a bitwise AND between elements of the second source vector and the inverted corresponding elements of the third source vector, then an exclusive-OR between the results and the corresponding elements of the first source vector. The final results are destructively placed in the corresponding elements of the destination and first source vector. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let m : integer = UInt(Zm); let k : integer = UInt(Zk); let dn : integer = UInt(Zdn);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let operand1 : bits(VL) = Z{}(dn); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = Z{}(k); [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = operand1 XOR (operand2 AND NOT(operand3));
