## PNEXT

Find next active predicate This instruction constructs a loop that iterates over all true elements in the vector select predicate register. If all elements in the first source predicate register are false it determines the first true element in the vector select predicate register, otherwise it determines the next true element in the vector select predicate register that follows the last true element in the first source predicate register. All elements of the destination predicate register are set to false, except the element corresponding to the determined vector select element, if any, which is set to true. This instruction sets the First (N), None (Z), and !Last (C) condition flags based on the predicate result, and sets the V flag to zero.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let v : integer = UInt(Pv); let dn : integer = UInt(Pdn);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer{} = VL DIV esize; let mask : bits(PL) = P{}(v); let operand : bits(PL) = P{}(dn); var result : bits(PL) = Zeros{PL}; let psize : integer{} = esize DIV 8; var next : integer = [LastActiveElement](shared_pseudocode.html#func_LastActiveElement_3){PL}(operand, esize) + 1; while next < elements && (\![ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, next, esize)) looplimit elements do next = next + 1; end; if next < elements then result[next\*:psize] = ZeroExtend{psize}('1'); end; PSTATE.[N,Z,C,V] = [PredTest](shared_pseudocode.html#func_PredTest_4){PL}(mask, result, esize); [P](shared_pseudocode.html#accessor_P_2){PL}(dn) = result;
