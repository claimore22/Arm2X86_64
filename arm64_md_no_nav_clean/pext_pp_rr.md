## PEXT (predicate pair)

Predicate extract pair from predicate-as-counter This instruction converts the source predicate-as-counter into a four register wide predicate-as-mask, and copies the two portions of the mask value selected by the portion index to the two destination predicate registers. A portion corresponds to a one predicate register fraction of the wider predicate-as-mask value.

#### Decode

if !IsFeatureImplemented(FEAT_SME2) && !IsFeatureImplemented(FEAT_SVE2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt('1'::PNn); let d0 : integer = UInt(Pd); let d1 : integer = (UInt(Pd) + 1) MOD 16; let part : integer = UInt(i1);

### Operation

if IsFeatureImplemented(FEAT_SVE2p1) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let pred : bits(PL) = P{}(n); let mask : bits(PL\*4) = CounterToPredicate{}(pred[15:0]); var result0 : bits(PL); var result1 : bits(PL); let psize : integer{} = esize DIV 8; for e = 0 to elements-1 do let pbit : bit = [PredicateElement](shared_pseudocode.html#func_PredicateElement_4){PL\*4}(mask, part \* 2 \* elements + e, esize); result0[e\*:psize] = ZeroExtend{psize}(pbit); end; for e = 0 to elements-1 do let pbit : bit = [PredicateElement](shared_pseudocode.html#func_PredicateElement_4){PL\*4}(mask, part \* 2 \* elements + elements + e, esize); result1[e\*:psize] = ZeroExtend{psize}(pbit); end; [P](shared_pseudocode.html#accessor_P_2){PL}(d0) = result0; [P](shared_pseudocode.html#accessor_P_2){PL}(d1) = result1;
