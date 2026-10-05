## RAX1

Bitwise rotate left by 1 and exclusive-OR This instruction rotates each 64-bit element of the second source vector left by 1 and performs an exclusive-OR with the corresponding elements of the first source vector. The results are placed in the corresponding elements of the destination vector. This instruction is unpredicated. ID_AA64ZFR0_EL1.SHA3 indicates whether this instruction is implemented. This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled, or FEAT_SME2p1 is implemented.

#### Decode

if !IsFeatureImplemented(FEAT_SVE_SHA3) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd);

### Operation

if IsFeatureImplemented(FEAT_SME2p1) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 64; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); for e = 0 to elements-1 do let element1 : bits(64) = operand1[e\*:64]; let element2 : bits(64) = operand2[e\*:64]; result[e\*:64] = element1 XOR ROL(element2, 1); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
