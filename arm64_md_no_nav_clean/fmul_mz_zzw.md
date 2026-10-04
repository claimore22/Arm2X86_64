## FMUL (multiple vectors)

Multi-vector floating-point multiply This instruction multiplies all the floating-point elements of the two or four first source vectors by the corresponding floating-point elements of the two or four second source vectors, and places the results in the corresponding elements of the two or four destination vectors. This instruction follows SME2 floating-point numerical behaviors corresponding to instructions that place their results in one or more SVE Z vectors. This instruction is unpredicated. It has encodings from 2 classes: [Two registers](#iclass_two_registers) and [Four registers](#iclass_four_registers)

#### Decode

if !IsFeatureImplemented(FEAT_SME2p2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let d : integer = UInt(Zd::'00'); let n : integer = UInt(Zn::'00'); let m : integer = UInt(Zm::'00'); let nreg : integer{} = 4;

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; var results : array [[4]] of bits(VL); for r = 0 to nreg-1 do let operand1 : bits(VL) = Z{}(n+r); let operand2 : bits(VL) = Z{}(m+r); for e = 0 to elements-1 do let element1 : bits(esize) = operand1[e\*:(esize)]; let element2 : bits(esize) = operand2[e\*:(esize)]; results[[r]][e\*:(esize)] = [FPMul](shared_pseudocode.html#func_FPMul_4){esize}(element1, element2, FPCR()); end; end; for r = 0 to nreg-1 do [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+r) = results[[r]]; end;
