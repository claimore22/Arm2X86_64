## SQDECP (scalar)

Signed saturating decrement scalar by count of true predicate elements This instruction determines the number of true elements in the source predicate and then uses the result to decrement the scalar destination. The result is saturated to the source general-purpose register's signed integer range. A 32-bit saturated result is then sign-extended to 64 bits. It has encodings from 2 classes: [32-bit](#iclass_32_bit) and [64-bit](#iclass_64_bit)

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let m : integer = UInt(Pm); let dn : integer = UInt(Rdn); let unsigned : boolean = FALSE; let ssize : integer{} = 64;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let operand1 : bits(ssize) = X{}(dn); let operand2 : bits(PL) = P{}(m); var result : bits(ssize); var count : integer = 0; for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(operand2, e, esize) then count = count + 1; end; end; if unsigned then (result, -) = [UnsignedSatQ](shared_pseudocode.html#func_UnsignedSatQ_2){ssize}(UInt(operand1) - count); [X](shared_pseudocode.html#accessor_X_2){64}(dn) = ZeroExtend{64}(result); else (result, -) = [SignedSatQ](shared_pseudocode.html#func_SignedSatQ_2){ssize}(SInt(operand1) - count); [X](shared_pseudocode.html#accessor_X_2){64}(dn) = SignExtend{64}(result); end;
