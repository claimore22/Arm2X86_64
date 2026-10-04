## ADDHA

Add horizontally vector elements to ZA tile This instruction adds each element of the source vector to the corresponding Active element of each horizontal slice of a ZA tile. The tile elements are predicated by a pair of governing predicates. An element of a horizontal slice is considered active if its corresponding element in the second governing predicate is TRUE and the element corresponding to its horizontal slice number in the first governing predicate is TRUE. Inactive elements in the destination tile remain unmodified. ID_AA64SMFR0_EL1.I16I64 indicates whether the 64-bit integer variant is implemented. It has encodings from 2 classes: [32-bit](#iclass_32_bit) and [64-bit](#iclass_64_bit)

#### Decode

if !IsFeatureImplemented(FEAT_SME_I16I64) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 64; let a : integer = UInt(Pn); let b : integer = UInt(Pm); let n : integer = UInt(Zn); let da : integer = UInt(ZAda);

### Operation

[CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let dim : integer{} = VL DIV esize; let mask1 : bits(PL) = P{}(a); let mask2 : bits(PL) = P{}(b); let operand_src : bits(VL) = Z{}(n); let operand_acc : bits(dim\*dim\*esize) = ZAtile{}(da, esize); var result : bits(dim\*dim\*esize); for col = 0 to dim-1 do let element : bits(esize) = operand_src[col\*:esize]; for row = 0 to dim-1 do var res : bits(esize) = operand_acc[(row\*dim+col)\*:esize]; if ([ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask1, row, esize) && [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask2, col, esize)) then res = res + element; end; result[(row\*dim+col)\*:esize] = res; end; end; [ZAtile](shared_pseudocode.html#accessor_ZAtile_3){dim\*dim\*esize}(da, esize) = result;
