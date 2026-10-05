## PUNPKHI, PUNPKLO

Unpack and widen half of predicate This instruction unpacks elements from the lowest or highest half of the source predicate and places them in elements of twice their size within the destination predicate. This instruction is unpredicated. It has encodings from 2 classes: [High half](#iclass_high_half) and [Low half](#iclass_low_half)

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 16; let n : integer = UInt(Pn); let d : integer = UInt(Pd); let hi : boolean = FALSE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let operand : bits(PL) = P{}(n); var result : bits(PL); let psize : integer{} = esize DIV 8; for e = 0 to elements-1 do let pbit : bit = [PredicateElement](shared_pseudocode.html#func_PredicateElement_4){PL}(operand, if hi then e + elements else e, esize DIV 2); result[e\*:psize] = ZeroExtend{psize}(pbit); end; [P](shared_pseudocode.html#accessor_P_2){PL}(d) = result;
