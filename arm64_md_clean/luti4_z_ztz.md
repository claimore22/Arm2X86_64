## LUTI4 (single)

Lookup table read with 4-bit indexes (single) This instruction copies 8-bit, 16-bit or 32-bit elements from ZT0 to one destination vector using packed 4-bit indices from a segment of the source vector register. A segment corresponds to a portion of the source vector that is consumed in order to fill the destination vector. The segment is selected by the vector segment index modulo the total number of segments. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '11' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let isize : integer{} = 4; let n : integer = UInt(Zn); let d : integer = UInt(Zd); let imm : integer = UInt(i3); let nreg : integer{} = 1;

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); [CheckSMEZT0Enabled](shared_pseudocode.html#func_CheckSMEZT0Enabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let segments : integer = (esize DIV (isize \* nreg)) as integer{1, 2, 4, 8, 16}; let segment : integer = imm MOD segments; let indexes : bits(VL) = Z{}(n); let dst : integer = d; let table : bits(512) = ZT0{}(); for r = 0 to nreg-1 do let base : integer = (segment \* nreg + r) \* elements; var result : bits(VL); for e = 0 to elements-1 do let index : integer = UInt(indexes[(base+e)\*:isize]); result[e\*:esize] = table[index\*:32][esize-1:0]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dst+r) = result; end;
