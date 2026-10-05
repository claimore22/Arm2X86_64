## ZIP (two registers)

Interleave elements from two vectors This instruction places the two-way interleaved elements from the first and second source vectors in the corresponding elements of the two destination vectors. This instruction is unpredicated. It has encodings from 2 classes: [8-bit to 64-bit elements](#iclass_8_bit_to_64_bit_elements) and [128-bit element](#iclass_128_bit_element)

#### Decode

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if [MaxImplementedSVL](shared_pseudocode.html#func_MaxImplementedSVL_0)() < 256 then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 128; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd::'0');

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); if VL < esize \* 2 then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let pairs : integer = VL DIV (esize \* 2); let operand0 : bits(VL) = Z{}(n); let operand1 : bits(VL) = Z{}(m); var result : bits(VL); for r = 0 to 1 do let base : integer = r \* pairs; for p = 0 to pairs-1 do result[(2\*p+0)\*:esize] = operand0[(base+p)\*:esize]; result[(2\*p+1)\*:esize] = operand1[(base+p)\*:esize]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+r) = result; end;
