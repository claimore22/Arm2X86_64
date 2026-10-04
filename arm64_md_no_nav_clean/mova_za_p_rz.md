## MOVA (vector to tile, single)

Move Z vector to ZA tile slice This instruction operates on individual horizontal or vertical slices within a named ZA tile of the specified element size. The slice number within the tile is selected by the sum of the slice index register and immediate offset, modulo the number of such elements in a vector. The immediate offset is in the range 0 to the number of elements in a 128-bit vector segment minus 1. Inactive elements in the destination slice remain unmodified. This instruction is used by the alias [MOV (vector to tile, single)](mov_mova_za_p_rz.html "Move Z vector to ZA tile slice"). It has encodings from 5 classes: [8-bit](#iclass_8_bit) , [16-bit](#iclass_16_bit) , [32-bit](#iclass_32_bit) , [64-bit](#iclass_64_bit) and [128-bit](#iclass_128_bit)

#### Decode

if !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let g : integer = UInt(Pg); let s : integer = UInt('011'::Rs); let n : integer = UInt(Zn); let d : integer = UInt(ZAd); let offset : integer = 0; let esize : integer{} = 128; let vertical : boolean = V == '1';

### Operation

[CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let dim : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand : bits(VL) = Z{}(n); let index : bits(32) = X{}(s); let slice : integer = (UInt(index) + offset) MOD dim; var result : bits(VL) = ZAslice{}(d, esize, vertical, slice); for e = 0 to dim-1 do let element : bits(esize) = operand[e\*:esize]; if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then result[e\*:esize] = element; end; end; [ZAslice](shared_pseudocode.html#accessor_ZAslice_5){VL}(d, esize, vertical, slice) = result;
