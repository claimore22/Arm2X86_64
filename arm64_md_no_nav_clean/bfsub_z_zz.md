## BFSUB (unpredicated)

BFloat16 subtract (unpredicated) This instruction subtracts all BFloat16 elements of the second source vector from corresponding BFloat16 elements of the first source vector and places the results in the corresponding elements of the destination vector. This instruction follows SVE2 non-widening BFloat16 numerical behaviors. This instruction is unpredicated. ID_AA64ZFR0_EL1.B16B16 indicates whether this instruction is implemented.

#### Decode

if !IsFeatureImplemented(FEAT_SVE_B16B16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd);

### Operation

if IsFeatureImplemented(FEAT_SME2) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 16; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); for e = 0 to elements-1 do let element1 : bits(16) = operand1[e\*:16]; let element2 : bits(16) = operand2[e\*:16]; result[e\*:16] = [BFSub](shared_pseudocode.html#func_BFSub_3)(element1, element2, FPCR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
