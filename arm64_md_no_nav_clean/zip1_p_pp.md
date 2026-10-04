## ZIP1, ZIP2 (predicates)

Interleave elements from two half predicates This instruction interleaves alternating elements from the lowest or highest halves of the first and second source predicates, and places the values in the destination predicate. This instruction is unpredicated. It has encodings from 2 classes: [High halves](#iclass_high_halves) and [Low halves](#iclass_low_halves)

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Pn); let m : integer = UInt(Pm); let d : integer = UInt(Pd); let part : integer = 0;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let pairs : integer = VL DIV (esize \* 2); let operand1 : bits(PL) = P{}(n); let operand2 : bits(PL) = P{}(m); var result : bits(PL); let base : integer = part \* pairs; for p = 0 to pairs-1 do result[(2\*p+0)\*:(esize DIV 8)] = operand1[(base+p)\*:(esize DIV 8)]; result[(2\*p+1)\*:(esize DIV 8)] = operand2[(base+p)\*:(esize DIV 8)]; end; [P](shared_pseudocode.html#accessor_P_2){PL}(d) = result;
