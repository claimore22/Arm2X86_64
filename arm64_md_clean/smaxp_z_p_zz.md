## SMAXP

Signed maximum pairwise This instruction calculates the maximum value of each pair of adjacent signed integer elements within each source vector, and interleaves the results from corresponding lanes. The interleaved result values are destructively placed in the first source vector.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let m : integer = UInt(Zm); let dn : integer = UInt(Zdn);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand1 : bits(VL) = Z{}(dn); let operand2 : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(m) else Zeros{VL}; var result : bits(VL); var element1 : integer; var element2 : integer; for e = 0 to elements-1 do if \![ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then result[e\*:esize] = operand1[e\*:esize]; else if IsEven(e) then element1 = SInt(operand1[(e + 0)\*:esize]); element2 = SInt(operand1[(e + 1)\*:esize]); else element1 = SInt(operand2[(e - 1)\*:esize]); element2 = SInt(operand2[(e + 0)\*:esize]); end; let res : integer = Max(element1, element2); result[e\*:esize] = res[esize-1:0]; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;
