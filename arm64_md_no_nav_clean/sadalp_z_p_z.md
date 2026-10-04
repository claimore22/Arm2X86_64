## SADALP

Signed add and accumulate long pairwise This instruction adds pairs of adjacent signed integer values and accumulates the results into the overlapping double-width elements of the destination vector.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Zn); let da : integer = UInt(Zda);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand_acc : bits(VL) = Z{}(da); let operand_src : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(n) else Zeros{VL}; var result : bits(VL); for e = 0 to elements-1 do if \![ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then result[e\*:esize] = operand_acc[e\*:esize]; else let element1 : integer = SInt(operand_src[(2\*e + 0)\*:(esize DIV 2)]); let element2 : integer = SInt(operand_src[(2\*e + 1)\*:(esize DIV 2)]); let sum : bits(esize) = (element1 + element2)[esize-1:0]; result[e\*:esize] = operand_acc[e\*:esize] + sum; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;
