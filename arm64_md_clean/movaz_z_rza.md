## MOVAZ (tile to vector, single)

Move and zero ZA tile slice to Z vector This instruction operates on a horizontal or vertical slice within a named ZA tile of the specified element size. The tile slice is zeroed after moving its contents to the destination vector. The slice number within the tile is selected by the sum of the slice index register and immediate offset, modulo the number of such elements in a vector. The immediate offset is in the range 0 to the number of elements in a 128-bit vector segment minus 1. This instruction is unpredicated. It has encodings from 5 classes: [8-bit](#iclass_8_bit) , [16-bit](#iclass_16_bit) , [32-bit](#iclass_32_bit) , [64-bit](#iclass_64_bit) and [128-bit](#iclass_128_bit)

#### Decode

if !IsFeatureImplemented(FEAT_SME2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let s : integer = UInt('011'::Rs); let n : integer = UInt(ZAn); let offset : integer = 0; let esize : integer{} = 128; let d : integer = UInt(Zd); let vertical : boolean = V == '1';

### Operation

[CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let dim : integer = VL DIV esize; let index : bits(32) = X{}(s); let slice : integer = (UInt(index) + offset) MOD dim; let operand : bits(VL) = ZAslice{}(n, esize, vertical, slice); [ZAslice](shared_pseudocode.html#accessor_ZAslice_5){VL}(n, esize, vertical, slice) = Zeros{VL}; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = operand;
