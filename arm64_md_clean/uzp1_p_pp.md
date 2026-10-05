## UZP1, UZP2 (predicates)

Concatenate even or odd elements from two predicates This instruction concatenates adjacent even or odd-numbered elements from the first and second source predicates, and places the values in the destination predicate. This instruction is unpredicated. It has encodings from 2 classes: [Even](#iclass_even) and [Odd](#iclass_odd)

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Pn); let m : integer = UInt(Pm); let d : integer = UInt(Pd); let part : integer = 1;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let pairs : integer = VL DIV (esize \* 2); let operand1 : bits(PL) = P{}(n); let operand2 : bits(PL) = P{}(m); var result : bits(PL); for p = 0 to pairs - 1 do result[p\*:(esize DIV 8)] = operand1[(2\*p+part)\*:(esize DIV 8)]; end; for p = 0 to pairs - 1 do result[(pairs+p)\*:(esize DIV 8)] = operand2[(2\*p+part)\*:(esize DIV 8)]; end; [P](shared_pseudocode.html#accessor_P_2){PL}(d) = result;
