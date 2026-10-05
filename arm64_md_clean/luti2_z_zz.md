## LUTI2 (8-bit and 16-bit)

Lookup table read with 2-bit indices (8-bit and 16-bit) This instruction copies indexed 8-bit or 16-bit elements from the low 128 bits of the table vector to the destination vector using packed 2-bit indices from a segment of the source vector. A segment corresponds to a portion of the source vector that is consumed in order to fill the destination vector. The segment is selected by the vector segment index. This instruction is unpredicated. It has encodings from 2 classes: [Byte](#iclass_byte) and [Halfword](#iclass_halfword)

#### Decode

if ((!IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME2)) \|\| !IsFeatureImplemented(FEAT_LUT)) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let isize : integer{} = 2; let esize : integer{} = 16; let m : integer = UInt(Zm); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let part : integer = UInt(i3h::i3l);

### Operation

if IsFeatureImplemented(FEAT_SME2) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let ibase : integer = elements \* part; let indexes : bits(VL) = Z{}(m); let table : bits(VL) = Z{}(n); var result : bits(VL); for e = 0 to elements-1 do let index : integer = UInt(indexes[(ibase + e)\*:isize]); result[e\*:esize] = table[index\*:esize]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
