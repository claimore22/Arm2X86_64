## SCLAMP

Multi-vector signed clamp to minimum/maximum This instruction clamps each signed element in the two or four destination vectors to between the signed minimum value in the corresponding element of the first source vector and the signed maximum value in the corresponding element of the second source vector and destructively places the clamped results in the corresponding elements of the two or four destination vectors. This instruction is unpredicated. It has encodings from 2 classes: [Two registers](#iclass_two_registers) and [Four registers](#iclass_four_registers)

#### Decode

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd::'00'); let nreg : integer{} = 4;

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; var results : array [[4]] of bits(VL); for r = 0 to nreg-1 do let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = Z{}(d+r); for e = 0 to elements-1 do let element1 : integer = SInt(operand1[e\*:esize]); let element2 : integer = SInt(operand2[e\*:esize]); let element3 : integer = SInt(operand3[e\*:esize]); let res : integer = Min(Max(element1, element3), element2); results[[r]][e\*:esize] = res[esize-1:0]; end; end; for r = 0 to nreg-1 do [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+r) = results[[r]]; end;
