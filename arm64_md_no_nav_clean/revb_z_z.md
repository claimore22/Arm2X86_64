## REVB, REVH, REVW

Reverse bytes / halfwords / words within elements (predicated) This instruction reverses the order of 8-bit bytes, 16-bit halfwords or 32-bit words within each active element of the source vector, and places the results in the corresponding elements of the destination vector. Inactive elements in the destination vector register remain unmodified or are set to zero, depending on whether merging or zeroing predication is selected. It has encodings from 6 classes: [Byte, merging](#iclass_byte_merging) , [Byte, zeroing](#iclass_byte_zeroing) , [Halfword, merging](#iclass_halfword_merging) , [Halfword, zeroing](#iclass_halfword_zeroing) , [Word, merging](#iclass_word_merging) and [Word, zeroing](#iclass_word_zeroing)

#### Decode

if !IsFeatureImplemented(FEAT_SVE2p2) && !IsFeatureImplemented(FEAT_SME2p2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size != '11' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let swsize : integer = 32; let merging : boolean = FALSE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(n) else Zeros{VL}; var result : bits(VL) = if merging then [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) else Zeros{VL}; for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then let element : bits(esize) = operand[e\*:esize]; result[e\*:esize] = [Reverse](shared_pseudocode.html#func_Reverse_3){esize}(element, swsize); end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
