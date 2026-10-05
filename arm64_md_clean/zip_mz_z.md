## ZIP (four registers)

Interleave elements from four vectors This instruction places the four-way interleaved elements from the four source vectors in the corresponding elements of the four destination vectors. This instruction is unpredicated. It has encodings from 2 classes: [8-bit to 64-bit elements](#iclass_8_bit_to_64_bit_elements) and [128-bit element](#iclass_128_bit_element)

#### Decode

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if [MaxImplementedSVL](shared_pseudocode.html#func_MaxImplementedSVL_0)() < 512 then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 128; let n : integer = UInt(Zn::'00'); let d : integer = UInt(Zd::'00');

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); if VL < esize \* 4 then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let quads : integer = VL DIV (esize \* 4); let operand0 : bits(VL) = Z{}(n); let operand1 : bits(VL) = Z{}(n+1); let operand2 : bits(VL) = Z{}(n+2); let operand3 : bits(VL) = Z{}(n+3); var result : bits(VL); for r = 0 to 3 do let base : integer = r \* quads; for q = 0 to quads-1 do result[(4\*q+0)\*:esize] = operand0[(base+q)\*:esize]; result[(4\*q+1)\*:esize] = operand1[(base+q)\*:esize]; result[(4\*q+2)\*:esize] = operand2[(base+q)\*:esize]; result[(4\*q+3)\*:esize] = operand3[(base+q)\*:esize]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+r) = result; end;
