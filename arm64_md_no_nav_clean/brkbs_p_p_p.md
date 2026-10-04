## BRKBS

Break before first true condition, setting the condition flags This instruction sets the destination predicate elements up to but not including the first active and true source element to true, then sets subsequent elements to false. Inactive elements in the destination predicate register are set to zero. This instruction sets the First (N), None (Z), and !Last (C) condition flags based on the predicate result, and sets the V flag to zero.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8; let g : integer = UInt(Pg); let n : integer = UInt(Pn); let d : integer = UInt(Pd); let merging : boolean = FALSE; let setflags : boolean = TRUE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand : bits(PL) = P{}(n); let operand2 : bits(PL) = if merging then [P](shared_pseudocode.html#accessor_P_2){PL}(d) else Zeros{PL}; var break : boolean = FALSE; var result : bits(PL); let psize : integer{} = esize DIV 8; for e = 0 to elements-1 do let element : boolean = [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(operand, e, esize); if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then break = break \|\| element; let pbit : bit = if !break then '1' else '0'; result[e\*:psize] = ZeroExtend{psize}(pbit); elsif merging then let pbit : bit = [PredicateElement](shared_pseudocode.html#func_PredicateElement_4){PL}(operand2, e, esize); result[e\*:psize] = ZeroExtend{psize}(pbit); else result[e\*:psize] = ZeroExtend{psize}('0'); end; end; if setflags then PSTATE.[N,Z,C,V] = [PredTest](shared_pseudocode.html#func_PredTest_4){PL}(mask, result, esize); end; [P](shared_pseudocode.html#accessor_P_2){PL}(d) = result;
