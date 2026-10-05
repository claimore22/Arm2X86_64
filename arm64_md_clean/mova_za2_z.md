## MOVA (vector to tile, two registers)

Move Z two-vector operand to ZA two-slice operand This instruction operates on a horizontal or vertical ZA two-slice operand within a ZA tile of the specified element size. The first slice of the two-slice operand is selected by rounding down the sum of the slice index register and the immediate offset to the nearest lower multiple of 2, modulo the number of slices in the tile. The immediate offset is a multiple of 2 in the range 0 to the number of elements in a 128-bit vector segment minus 2. This instruction is unpredicated. This instruction is used by the alias [MOV (vector to tile, two registers)](mov_mova_za2_z.html "Move Z two-vector operand to ZA two-slice operand"). It has encodings from 4 classes: [8-bit](#iclass_8_bit) , [16-bit](#iclass_16_bit) , [32-bit](#iclass_32_bit) and [64-bit](#iclass_64_bit)

#### Decode

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let s : integer = UInt('011'::Rs); let nreg : integer{} = 2; let esize : integer{} = 64; let n : integer = UInt(Zn::'0'); let d : integer = UInt(ZAd); let offset : integer = 0; let vertical : boolean = V == '1';

### Operation

[CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); if nreg == 4 && esize == 64 && VL < 256 then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let slices : integer = VL DIV esize; let index : bits(32) = X{}(s); let slice : integer = ((UInt(index) - (UInt(index) MOD nreg)) + offset) MOD slices; for r = 0 to nreg-1 do let result : bits(VL) = Z{}(n + r); [ZAslice](shared_pseudocode.html#accessor_ZAslice_5){VL}(d, esize, vertical, slice + r) = result; end;
