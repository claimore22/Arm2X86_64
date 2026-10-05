## EOR3

Bitwise exclusive-OR between three vectors This instruction performs a bitwise exclusive-OR between the corresponding elements of the three source vectors, and destructively places the results in the corresponding elements of the destination and first source vector. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let m : integer = UInt(Zm); let k : integer = UInt(Zk); let dn : integer = UInt(Zdn);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let operand1 : bits(VL) = Z{}(dn); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = Z{}(k); [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = operand1 XOR operand2 XOR operand3;
