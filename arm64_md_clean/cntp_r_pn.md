## CNTP (predicate as counter)

Set scalar to count from predicate-as-counter This instruction determines the number of true elements in the source predicate, up to the total number of elements in two or four vectors, and places the scalar result in the destination general-purpose register.

#### Decode

if !IsFeatureImplemented(FEAT_SME2) && !IsFeatureImplemented(FEAT_SVE2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(PNn); let d : integer = UInt(Rd); let width : integer = 2 << UInt(vl);

### Operation

if IsFeatureImplemented(FEAT_SVE2p1) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let pred : bits(PL) = P{}(n); let mask : bits(PL\*4) = CounterToPredicate{}(pred[15:0]); var sum : bits(64) = Zeros{}; let maxelements : integer = elements \* width; for e = 0 to maxelements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL\*4}(mask, e, esize) then sum = sum + 1; end; end; [X](shared_pseudocode.html#accessor_X_2){64}(d) = sum;
