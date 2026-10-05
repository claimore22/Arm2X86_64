## PTRUE (predicate)

Initialize predicate from named constraint This instruction sets elements of the destination predicate to true if the element number satisfies the named predicate constraint, or to false otherwise. If the constraint specifies more elements than are available at the current vector length, then all elements of the destination predicate are set to false. The named predicate constraint limits the number of active elements in a single predicate to: - A fixed number (VL1 to VL256) - The largest power of two (POW2) - The largest multiple of three or four (MUL3 or MUL4) - All available, implicitly a multiple of two (ALL). Unspecified or out of range constraint encodings generate an empty predicate or zero element count rather than Undefined Instruction exception. This instruction does not set the condition flags.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let d : integer = UInt(Pd); let setflags : boolean = FALSE; let pat : bits(5) = pattern;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let count : integer = [DecodePredCount](shared_pseudocode.html#func_DecodePredCount_2)(pat, esize); var result : bits(PL); let psize : integer{} = esize DIV 8; for e = 0 to elements-1 do let pbit : bit = if e < count then '1' else '0'; result[e\*:psize] = ZeroExtend{psize}(pbit); end; if setflags then PSTATE.[N,Z,C,V] = [PredTest](shared_pseudocode.html#func_PredTest_4){PL}(result, result, esize); end; [P](shared_pseudocode.html#accessor_P_2){PL}(d) = result;
