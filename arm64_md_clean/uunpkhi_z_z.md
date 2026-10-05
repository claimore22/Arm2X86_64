## UUNPKHI, UUNPKLO

Unsigned unpack and extend half of vector This instruction unpacks and zero-extends elements from the lowest or highest half of the source vector, and writes the zero-extended values into double-width elements of the destination vector. This instruction is unpredicated. It has encodings from 2 classes: [High half](#iclass_high_half) and [Low half](#iclass_low_half)

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let unsigned : boolean = TRUE; let hi : boolean = FALSE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let hsize : integer{} = esize DIV 2; let offset : integer = if hi then elements else 0; let operand : bits(VL) = Z{}(n); var result : bits(VL); for e = 0 to elements-1 do let element : bits(hsize) = operand[(e + offset)\*:hsize]; result[e\*:esize] = Extend{esize}(element, unsigned); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
