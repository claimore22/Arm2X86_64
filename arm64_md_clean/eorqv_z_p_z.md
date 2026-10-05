## EORQV

Bitwise exclusive-OR reduction of quadword vector segments This instruction performs a bitwise exclusive-OR between elements in the same position from each 128-bit source vector segment, and places each result into the corresponding element number of the 128-bit SIMD&FP destination register. Inactive elements in the source vector are treated as all zeros.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2p1) && !IsFeatureImplemented(FEAT_SME2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Vd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let segments : integer = VL DIV 128; let elempersegment : integer = 128 DIV esize; let mask : bits(PL) = P{}(g); let operand : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(n) else Zeros{VL}; var result : bits(128) = Zeros{}; var stmp : bits(128) = Zeros{}; var dtmp : bits(esize); for e = 0 to elempersegment-1 do dtmp = Zeros{esize}; for s = 0 to segments-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, s \* elempersegment + e, esize) then stmp = operand[s\*:128]; dtmp = dtmp XOR stmp[e\*:esize]; end; end; result[e\*:esize] = dtmp[esize-1:0]; end; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;
