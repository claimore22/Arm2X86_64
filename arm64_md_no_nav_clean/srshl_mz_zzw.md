## SRSHL (multiple vectors)

Multi-vector signed rounding shift left This instruction shifts the signed elements of the two or four first source vectors by the corresponding elements of the two or four second source vectors and destructively places the rounded results in the corresponding elements of the two or four first source vectors. A positive shift amount performs a left shift, otherwise a right shift by the negated shift amount is performed. This instruction is unpredicated. It has encodings from 2 classes: [Two registers](#iclass_two_registers) and [Four registers](#iclass_four_registers)

#### Decode

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let dn : integer = UInt(Zdn::'00'); let m : integer = UInt(Zm::'00'); let nreg : integer{} = 4;

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; var results : array [[4]] of bits(VL); for r = 0 to nreg-1 do let operand1 : bits(VL) = Z{}(dn+r); let operand2 : bits(VL) = Z{}(m+r); for e = 0 to elements-1 do let element : integer = SInt(operand1[e\*:esize]); var shift : integer = [ShiftSat](shared_pseudocode.html#func_ShiftSat_2)(SInt(operand2[e\*:esize]), esize); var res : integer; if shift >= 0 then res = element << shift; else shift = -shift; res = (element + (1 << (shift - 1))) >> shift; end; results[[r]][e\*:esize] = res[esize-1:0]; end; end; for r = 0 to nreg-1 do [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn+r) = results[[r]]; end;
