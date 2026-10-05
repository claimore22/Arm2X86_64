## CNTP (predicate)

Set scalar to count of true predicate elements This instruction determines the number of active and true elements in the source predicate and places the scalar result in the destination general-purpose register. Inactive predicate elements are not counted.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Pn); let d : integer = UInt(Rd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand : bits(PL) = P{}(n); var sum : bits(64) = Zeros{}; for e = 0 to elements-1 do if ([ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) && [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(operand, e, esize)) then sum = sum + 1; end; end; [X](shared_pseudocode.html#accessor_X_2){64}(d) = sum;
