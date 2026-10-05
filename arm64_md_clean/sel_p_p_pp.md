## SEL (predicates)

Conditionally select elements from two predicates This instruction reads Active elements from the first source predicate and Inactive elements from the second source predicate and places them in the corresponding elements of the destination predicate. This instruction does not set the condition flags. This instruction is used by the alias [MOV (predicate, merging)](mov_sel_p_p_pp.html "Move predicates (merging)").

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8; let g : integer = UInt(Pg); let n : integer = UInt(Pn); let m : integer = UInt(Pm); let d : integer = UInt(Pd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand1 : bits(PL) = P{}(n); let operand2 : bits(PL) = P{}(m); var result : bits(PL); let psize : integer{} = esize DIV 8; for e = 0 to elements-1 do let element1 : bit = [PredicateElement](shared_pseudocode.html#func_PredicateElement_4){PL}(operand1, e, esize); let element2 : bit = [PredicateElement](shared_pseudocode.html#func_PredicateElement_4){PL}(operand2, e, esize); if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then result[e\*:psize] = ZeroExtend{psize}(element1); else result[e\*:psize] = ZeroExtend{psize}(element2); end; end; [P](shared_pseudocode.html#accessor_P_2){PL}(d) = result;
