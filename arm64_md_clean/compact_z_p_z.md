## COMPACT

Copy Active vector elements to lower-numbered elements This instruction copies Active elements from the source vector to consecutive elements of the destination vector, in increasing order of element number, and sets any remaining elements to zero. This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled, or FEAT_SME2p2 is implemented. It has encodings from 2 classes: [Byte and halfword](#iclass_byte_and_halfword) and [Word and doubleword](#iclass_word_and_doubleword)

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME2p2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32 << UInt(sz); let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Zd);

### Operation

if IsFeatureImplemented(FEAT_SME2p2) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand1 : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(n) else Zeros{VL}; var result : bits(VL) = Zeros{}; var x : integer = 0; for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then let element : bits(esize) = operand1[e\*:esize]; result[x\*:esize] = element; x = x + 1; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
