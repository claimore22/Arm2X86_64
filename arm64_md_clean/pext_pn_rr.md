## PEXT (predicate)

Predicate extract from predicate-as-counter This instruction converts the source predicate-as-counter into a four register wide predicate-as-mask, and copies the portion of the mask value selected by the portion index to the destination predicate register. A portion corresponds to a one predicate register fraction of the wider predicate-as-mask value.

#### Decode

if !IsFeatureImplemented(FEAT_SME2) && !IsFeatureImplemented(FEAT_SVE2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt('1'::PNn); let d : integer = UInt(Pd); let part : integer = UInt(imm2);

### Operation

if IsFeatureImplemented(FEAT_SVE2p1) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let pred : bits(PL) = P{}(n); let mask : bits(PL\*4) = CounterToPredicate{}(pred[15:0]); var result : bits(PL); let psize : integer{} = esize DIV 8; for e = 0 to elements-1 do let pbit : bit = [PredicateElement](shared_pseudocode.html#func_PredicateElement_4){PL\*4}(mask, part \* elements + e, esize); result[e\*:psize] = ZeroExtend{psize}(pbit); end; [P](shared_pseudocode.html#accessor_P_2){PL}(d) = result;
