## ZERO (single-vector)

Zero ZA single-vector groups This instruction zeroes two or four ZA single-vector groups. The single-vector group within each half of or each quarter of the ZA array is selected by the sum of the vector select register and offset, modulo half or quarter the number of ZA array vectors. The vector group symbol, VGx2 or VGx4, indicates that the ZA operand consists of two or four ZA single-vector groups respectively. It has encodings from 2 classes: [Two ZA single-vectors](#iclass_two_za_single_vectors) and [Four ZA single-vectors](#iclass_four_za_single_vectors)

#### Decode

if !IsFeatureImplemented(FEAT_SME2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let v : integer = UInt('010'::Rv); let offset : integer = UInt(off3); let ngrp : integer = 4;

### Operation

[CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let vectors : integer = VL DIV 8; let vstride : integer = vectors DIV ngrp; let vbase : bits(32) = X{}(v); var vec : integer = (UInt(vbase) + offset) MOD vstride; for r = 0 to ngrp-1 do [ZAvector](shared_pseudocode.html#accessor_ZAvector_2){VL}(vec) = Zeros{VL}; vec = vec + vstride; end;
