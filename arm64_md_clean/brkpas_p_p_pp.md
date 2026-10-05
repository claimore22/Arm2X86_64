## BRKPAS

Break after first true condition, propagating from previous partition and setting the condition flags This instruction breaks after first true condition, propagating from previous partition, and sets the condition flags. If the Last active element of the first source predicate is false, then the destination predicate is set to all-false. Otherwise, the destination predicate elements up to and including the first active and true source element are set to true, then the subsequent elements are set to false. Inactive elements in the destination predicate register are set to zero. This instruction sets the First (N), None (Z), and !Last (C) condition flags based on the predicate result, and sets the V flag to zero.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8; let g : integer = UInt(Pg); let n : integer = UInt(Pn); let m : integer = UInt(Pm); let d : integer = UInt(Pd); let setflags : boolean = TRUE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand1 : bits(PL) = P{}(n); let operand2 : bits(PL) = P{}(m); var result : bits(PL); var last : boolean = ([LastActive](shared_pseudocode.html#func_LastActive_4){PL}(mask, operand1, 8) == '1'); for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, 8) then let pbit : bit = if last then '1' else '0'; result[e\*:1] = ZeroExtend{1}(pbit); last = last && (\![ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(operand2, e, 8)); else result[e\*:1] = ZeroExtend{1}('0'); end; end; if setflags then PSTATE.[N,Z,C,V] = [PredTest](shared_pseudocode.html#func_PredTest_4){PL}(mask, result, esize); end; [P](shared_pseudocode.html#accessor_P_2){PL}(d) = result;
