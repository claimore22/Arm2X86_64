## LUTI2

Lookup table read with 2-bit indices This instruction copies indexed 8-bit or 16-bit elements from the table vector to the destination vector using packed 2-bit indices from a segment of the source vector. A segment corresponds to a portion of the source vector that is consumed in order to fill the destination vector. The segment is selected by the vector segment index.

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) \|\| !IsFeatureImplemented(FEAT_LUT) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if op2 == '10' && op == '0' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let isize : integer{} = 2; let esize : integer{} = if op2 == '10' then 8 else 16; let part : integer = if op2 == '10' then UInt(len) else UInt(len::op);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let elements : integer = 128 DIV esize; let ibase : integer = elements \* part; let indices : bits(128) = V{}(m); let table : bits(128) = V{}(n); var result : bits(128); for e = 0 to elements-1 do let index : integer = UInt(indices[(ibase + e)\*:isize]); result[e\*:esize] = table[index\*:esize]; end; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;
