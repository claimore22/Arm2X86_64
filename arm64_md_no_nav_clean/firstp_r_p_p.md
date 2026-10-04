## FIRSTP

Scalar index of first true predicate element (predicated) This instruction determines the index of the first active and true element of the source predicate, and places the scalar result in the destination general-purpose register. If there are no active and true elements, the destination general-purpose register is updated to the value minus one.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2p2) && !IsFeatureImplemented(FEAT_SME2p2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Pn); let d : integer = UInt(Rd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand : bits(PL) = P{}(n); var index : integer = -1; for e = 0 to elements-1 do if (index == -1 && [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) && [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(operand, e, esize)) then index = e; end; end; [X](shared_pseudocode.html#accessor_X_2){64}(d) = index[63:0];
