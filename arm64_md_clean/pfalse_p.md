## PFALSE

Set all predicate elements to false This instruction sets all elements in the destination predicate to false. For programmer convenience, an assembler must also accept predicate-as-counter register name for the destination predicate register.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = UInt(Pd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; [P](shared_pseudocode.html#accessor_P_2){PL}(d) = Zeros{PL};
