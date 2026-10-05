## ZIP1, ZIP2 (vectors)

Interleave elements from two half vectors This instruction interleaves alternating elements from the lowest or highest halves of the first and second source vectors, and places the values in the destination vector. This instruction is unpredicated. The 128-bit element variant requires that the Effective SVE vector length is at least 256 bits. ID_AA64ZFR0_EL1.F64MM indicates whether the 128-bit element variant is implemented. The 128-bit element variant is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled. It has encodings from 4 classes: [High halves](#iclass_high_halves) , [High halves (quadwords)](#iclass_high_halves_quadwords_) , [Low halves](#iclass_low_halves) and [Low halves (quadwords)](#iclass_low_halves_quadwords_)

#### Decode

if !IsFeatureImplemented(FEAT_F64MM) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 128; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd); let part : integer = 0;

### Operation

if esize < 128 then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); if VL < esize \* 2 then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let pairs : integer = VL DIV (esize \* 2); let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL) = Zeros{}; let base : integer = part \* pairs; for p = 0 to pairs-1 do result[(2\*p+0)\*:esize] = operand1[(base+p)\*:esize]; result[(2\*p+1)\*:esize] = operand2[(base+p)\*:esize]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
