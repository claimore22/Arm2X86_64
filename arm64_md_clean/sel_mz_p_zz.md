## SEL

Multi-vector conditional select This instruction selects consecutive elements from the two or four first source vectors where vector select predicate elements are true, and places them in the corresponding elements of the two or four destination vectors. Where vector select predicate elements are false, the corresponding consecutive elements are selected from the two or four second source vectors and placed in the remaining elements of the destination vectors. It has encodings from 2 classes: [Two registers](#iclass_two_registers) and [Four registers](#iclass_four_registers)

#### Decode

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn::'00'); let m : integer = UInt(Zm::'00'); let d : integer = UInt(Zd::'00'); let v : integer = UInt('1'::PNv); let nreg : integer{} = 4;

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; var results : array[[4]] of bits(VL); let pred : bits(PL) = P{}(v); let mask : bits(PL \* nreg) = CounterToPredicate{}(pred[15:0]); for r = 0 to nreg-1 do let operand1 : bits(VL) = Z{}(n+r); let operand2 : bits(VL) = Z{}(m+r); for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL\*nreg}(mask, r \* elements + e, esize) then results[[r]][e\*:esize] = operand1[e\*:esize]; else results[[r]][e\*:esize] = operand2[e\*:esize]; end; end; end; for r = 0 to nreg-1 do [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+r) = results[[r]]; end;
