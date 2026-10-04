## ADD (to vector, multiple vectors)

Multi-vector add by vector to multi-vector This instruction adds elements of the second source vector to the corresponding elements of the two or four first source vectors and destructively places the results in the corresponding elements of the two or four first source vectors. This instruction is unpredicated. It has encodings from 2 classes: [Two registers](#iclass_two_registers) and [Four registers](#iclass_four_registers)

#### Decode

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let dn : integer = UInt(Zdn::'00'); let m : integer = UInt('0'::Zm); let nreg : integer{} = 4;

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; var results : array [[4]] of bits(VL); for r = 0 to nreg-1 do let operand1 : bits(VL) = Z{}(dn+r); let operand2 : bits(VL) = Z{}(m); for e = 0 to elements-1 do let element1 : bits(esize) = operand1[e\*:esize]; let element2 : bits(esize) = operand2[e\*:esize]; results[[r]][e\*:esize] = element1 + element2; end; end; for r = 0 to nreg-1 do [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn+r) = results[[r]]; end;
