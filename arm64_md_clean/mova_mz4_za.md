## MOVA (tile to vector, four registers)

Move ZA four-slice operand to Z four-vector operand This instruction operates on a horizontal or vertical ZA four-slice operand within a ZA tile of the specified element size. The first slice of the four-slice operand is selected by rounding down the sum of the slice index register and the immediate offset to the nearest lower multiple of 4, modulo the number of slices in the tile. The immediate offset is a multiple of 4 in the range 0 to the number of elements in a 128-bit vector segment minus 4. This instruction is unpredicated. This instruction is used by the alias [MOV (tile to vector, four registers)](mov_mova_mz4_za.html "Move ZA four-slice operand to Z four-vector operand"). It has encodings from 4 classes: [8-bit](#iclass_8_bit) , [16-bit](#iclass_16_bit) , [32-bit](#iclass_32_bit) and [64-bit](#iclass_64_bit)

#### Decode

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if [MaxImplementedSVL](shared_pseudocode.html#func_MaxImplementedSVL_0)() < 256 then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let s : integer = UInt('011'::Rs); let nreg : integer{} = 4; let esize : integer{} = 64; let d : integer = UInt(Zd::'00'); let n : integer = UInt(ZAn); let offset : integer = 0; let vertical : boolean = V == '1';

### Operation

[CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); if nreg == 4 && esize == 64 && VL < 256 then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let slices : integer = VL DIV esize; let index : bits(32) = X{}(s); let slice : integer = ((UInt(index) - (UInt(index) MOD nreg)) + offset) MOD slices; for r = 0 to nreg-1 do let result : bits(VL) = ZAslice{}(n, esize, vertical, slice + r); [Z](shared_pseudocode.html#accessor_Z_2){VL}(d + r) = result; end;
