## TBL

Programmable table lookup in one or two vector table (zeroing) This instruction reads each element of the second source (index) vector and uses its value to select an indexed element from a table of elements consisting of one or two consecutive vector registers, where the first or only vector holds the lower numbered elements, and places the indexed table element in the destination vector element corresponding to the index vector element. If an index value is greater than or equal to the number of vector elements, then it places zero in the corresponding destination vector element. Since the index values can select any element in a vector this operation is not naturally vector length agnostic. It has encodings from 2 classes: [Single register table](#iclass_single_register_table) and [Two register table](#iclass_two_register_table)

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd); let double_table : boolean = TRUE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let indexes : bits(VL) = Z{}(m); var result : bits(VL); let table_size : integer{} = if double_table then VL\*2 else VL; let table_elems : integer = table_size DIV esize; var table : bits(table_size); if double_table then let top : bits(VL) = Z{}((n + 1) MOD 32); let bottom : bits(VL) = Z{}(n); table = (top::bottom)[table_size-1:0]; else table = [Z](shared_pseudocode.html#accessor_Z_2){table_size}(n); end; for e = 0 to elements-1 do let idx : integer = UInt(indexes[e\*:esize]); result[e\*:esize] = if idx < table_elems then table[idx\*:esize] else Zeros{esize}; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
