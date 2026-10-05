## FRINTN

Multi-vector single-precision round to integral value, to nearest with ties to even This instruction rounds each single-precision element of the two or four source vectors to the nearest integral floating-point value, with ties rounding to an even value, and places the results in the corresponding elements of the two or four destination vectors. This instruction follows SME2 floating-point numerical behaviors corresponding to instructions that place their results in one or more SVE Z vectors. This instruction is unpredicated. It has encodings from 2 classes: [Two registers](#iclass_two_registers) and [Four registers](#iclass_four_registers)

#### Decode

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn::'00'); let d : integer = UInt(Zd::'00'); let nreg : integer{} = 4; let exact : boolean = FALSE; let rounding : [FPRounding](shared_pseudocode.html#type_FPRounding) = [FPRounding_TIEEVEN](shared_pseudocode.html#enum_FPRounding_TIEEVEN);

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 32; var results : array [[4]] of bits(VL); for r = 0 to nreg-1 do let operand : bits(VL) = Z{}(n+r); for e = 0 to elements-1 do let element : bits(32) = operand[e\*:32]; results[[r]][e\*:32] = [FPRoundInt](shared_pseudocode.html#func_FPRoundInt_5){32}(element, FPCR(), rounding, exact); end; end; for r = 0 to nreg-1 do [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+r) = results[[r]]; end;
