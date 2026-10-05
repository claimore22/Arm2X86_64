## INCP (scalar)

Increment scalar by count of true predicate elements This instruction determines the number of true elements in the source predicate and then uses the result to increment the scalar destination.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let m : integer = UInt(Pm); let dn : integer = UInt(Rdn);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let operand1 : bits(64) = X{}(dn); let operand2 : bits(PL) = P{}(m); var count : integer = 0; for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(operand2, e, esize) then count = count + 1; end; end; [X](shared_pseudocode.html#accessor_X_2){64}(dn) = operand1 + count;
