## FRINT64X

Floating-point round to 64-bit integer (predicated) This instruction rounds to integral floating-point values that fit into a 64-bit integer size using the rounding mode that is determined by the FPCR from each active floating-point element of the source vector, and places the results in the corresponding elements of the destination vector. Inactive elements in the destination vector register remain unmodified or are set to zero, depending on whether merging or zeroing predication is selected. It has encodings from 2 classes: [Merging](#iclass_merging) and [Zeroing](#iclass_zeroing)

#### Decode

if !IsFeatureImplemented(FEAT_SVE2p2) && !IsFeatureImplemented(FEAT_SME2p2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32 << UInt(sz); let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let intsize : integer{} = 64; let rounding : [FPRounding](shared_pseudocode.html#type_FPRounding) = [FPRoundingMode](shared_pseudocode.html#func_FPRoundingMode_1)(FPCR()); let merging : boolean = FALSE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(n) else Zeros{VL}; var result : bits(VL) = if merging then [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) else Zeros{VL}; for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then let element : bits(esize) = operand[e\*:esize]; result[e\*:esize] = [FPRoundIntN](shared_pseudocode.html#func_FPRoundIntN_5){esize}(element, FPCR(), rounding, intsize); end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
