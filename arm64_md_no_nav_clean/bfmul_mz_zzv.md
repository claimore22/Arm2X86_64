## BFMUL (multiple and single vector)

Multi-vector BFloat16 multiply by vector This instruction multiplies all the BFloat16 elements of the two or four first source vectors by the corresponding BFloat16 elements of the second source vector, and places the results in the corresponding elements of the two or four destination vectors. This instruction follows SME2 non-widening BFloat16 numerical behaviors corresponding to instructions that place their results in two or four SVE Z vectors. This instruction is unpredicated. It has encodings from 2 classes: [Two registers](#iclass_two_registers) and [Four registers](#iclass_four_registers)

#### Decode

if !IsFeatureImplemented(FEAT_SME2) \|\| !IsFeatureImplemented(FEAT_SVE_BFSCALE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = UInt(Zd::'00'); let n : integer = UInt(Zn::'00'); let m : integer = UInt('0'::Zm); let nreg : integer{} = 4;

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 16; var results : array [[4]] of bits(VL); for r = 0 to nreg-1 do let operand1 : bits(VL) = Z{}(n+r); let operand2 : bits(VL) = Z{}(m); for e = 0 to elements-1 do let element1 : bits(16) = operand1[e\*:(16)]; let element2 : bits(16) = operand2[e\*:(16)]; results[[r]][e\*:(16)] = [BFMul](shared_pseudocode.html#func_BFMul_3)(element1, element2, FPCR()); end; end; for r = 0 to nreg-1 do [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+r) = results[[r]]; end;
