## UQDECP (vector)

Unsigned saturating decrement vector by count of true predicate elements This instruction determines the number of true elements in the source predicate and then uses the result to decrement all destination vector elements. The results are saturated to the element unsigned integer range. The predicate size specifier may be omitted in assembler source code, but this is deprecated and will be prohibited in a future release of the architecture.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let m : integer = UInt(Pm); let dn : integer = UInt(Zdn); let unsigned : boolean = TRUE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(dn); let operand2 : bits(PL) = P{}(m); var result : bits(VL); var count : integer = 0; for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(operand2, e, esize) then count = count + 1; end; end; for e = 0 to elements-1 do let opelt : bits(esize) = operand1[e\*:esize]; if unsigned then (result[e\*:esize], -) = [UnsignedSatQ](shared_pseudocode.html#func_UnsignedSatQ_2){esize}(UInt(opelt) - count); else (result[e\*:esize], -) = [SignedSatQ](shared_pseudocode.html#func_SignedSatQ_2){esize}(SInt(opelt) - count); end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;
