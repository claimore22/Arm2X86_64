## ZERO (double-vector)

Zero ZA double-vector groups This instruction zeroes one, two, or four ZA double-vector groups. The double-vector group within all of, each half of, or each quarter of the ZA array is selected by the sum of the vector select register and offset range, modulo all, half, or quarter the number of ZA array vectors. The vector group symbol, VGx2 or VGx4, indicates that the ZA operand consists of two or four ZA double-vector groups respectively. It has encodings from 3 classes: [One ZA double-vector](#iclass_one_za_double_vector) , [Two ZA double-vectors](#iclass_two_za_double_vectors) and [Four ZA double-vectors](#iclass_four_za_double_vectors)

#### Decode

if !IsFeatureImplemented(FEAT_SME2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let v : integer = UInt('010'::Rv); let offset : integer = UInt(off2::'0'); let ngrp : integer = 4; let nvec : integer = 2;

### Operation

[CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let vectors : integer = VL DIV 8; let vstride : integer = vectors DIV ngrp; let vbase : bits(32) = X{}(v); var vec : integer = (UInt(vbase) + offset) MOD vstride; vec = vec - (vec MOD nvec); for r = 0 to ngrp-1 do for i = 0 to nvec-1 do [ZAvector](shared_pseudocode.html#accessor_ZAvector_2){VL}(vec + i) = Zeros{VL}; end; vec = vec + vstride; end;
