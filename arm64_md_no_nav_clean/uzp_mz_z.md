## UZP (four registers)

Concatenate elements from four vectors This instruction concatenates every fourth element from each of the four source vectors and places them in the corresponding elements of the four destination vectors. This instruction is unpredicated. It has encodings from 2 classes: [8-bit to 64-bit elements](#iclass_8_bit_to_64_bit_elements) and [128-bit element](#iclass_128_bit_element)

#### Decode

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if [MaxImplementedSVL](shared_pseudocode.html#func_MaxImplementedSVL_0)() < 512 then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 128; let n : integer = UInt(Zn::'00'); let d : integer = UInt(Zd::'00');

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); if VL < esize \* 4 then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let quads : integer = VL DIV (esize \* 4); var result0 : bits(VL); var result1 : bits(VL); var result2 : bits(VL); var result3 : bits(VL); for r = 0 to 3 do let operand : bits(VL) = Z{}(n+r); let base : integer = r \* quads; for q = 0 to quads-1 do result0[(base+q)\*:esize] = operand[(4\*q+0)\*:esize]; result1[(base+q)\*:esize] = operand[(4\*q+1)\*:esize]; result2[(base+q)\*:esize] = operand[(4\*q+2)\*:esize]; result3[(base+q)\*:esize] = operand[(4\*q+3)\*:esize]; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+0) = result0; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+1) = result1; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+2) = result2; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+3) = result3;
