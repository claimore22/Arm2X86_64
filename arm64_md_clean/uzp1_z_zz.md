## UZP1, UZP2 (vectors)

Concatenate even or odd elements from two vectors This instruction concatenates adjacent even or odd-numbered elements from the first and second source vectors, and places the values in the destination vector. This instruction is unpredicated. Note: UZP1 is equivalent to truncating and packing each element from two source vectors into a single destination vector with elements of half the size. The 128-bit element variant requires that the Effective SVE vector length is at least 256 bits. ID_AA64ZFR0_EL1.F64MM indicates whether the 128-bit element variant is implemented. The 128-bit element variant is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled. It has encodings from 4 classes: [Even](#iclass_even) , [Even (quadwords)](#iclass_even_quadwords_) , [Odd](#iclass_odd) and [Odd (quadwords)](#iclass_odd_quadwords_)

#### Decode

if !IsFeatureImplemented(FEAT_F64MM) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 128; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd); let part : integer = 1;

### Operation

if esize < 128 then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); if VL < esize \* 2 then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let pairs : integer = VL DIV (esize \* 2); let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL) = Zeros{}; for p = 0 to pairs - 1 do result[p\*:esize] = operand1[(2\*p+part)\*:esize]; end; for p = 0 to pairs - 1 do result[(pairs+p)\*:esize] = operand2[(2\*p+part)\*:esize]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
