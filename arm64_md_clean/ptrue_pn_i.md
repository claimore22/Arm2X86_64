## PTRUE (predicate as counter)

Initialize predicate-as-counter to all active This instruction sets the destination predicate as all Active elements, using the predicate-as-counter encoding.

#### Decode

if !IsFeatureImplemented(FEAT_SME2) && !IsFeatureImplemented(FEAT_SVE2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let d : integer = UInt('1'::PNd);

### Operation

if IsFeatureImplemented(FEAT_SVE2p1) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let result : bits(PL) = EncodePredCount{}(esize, elements, elements, FALSE); [P](shared_pseudocode.html#accessor_P_2){PL}(d) = result;
