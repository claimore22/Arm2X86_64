## DECD, DECH, DECW (vector)

Decrement vector by multiple of predicate constraint element count This instruction determines the number of active elements implied by the named predicate constraint, multiplies that by an immediate in the range 1 to 16 inclusive, and then uses the result to decrement all destination vector elements. The named predicate constraint limits the number of active elements in a single predicate to: - A fixed number (VL1 to VL256) - The largest power of two (POW2) - The largest multiple of three or four (MUL3 or MUL4) - All available, implicitly a multiple of two (ALL). Unspecified or out of range constraint encodings generate an empty predicate or zero element count rather than Undefined Instruction exception. It has encodings from 3 classes: [Doubleword](#iclass_doubleword) , [Halfword](#iclass_halfword) and [Word](#iclass_word)

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let dn : integer = UInt(Zdn); let pat : bits(5) = pattern; let imm : integer = UInt(imm4) + 1;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let count : integer = [DecodePredCount](shared_pseudocode.html#func_DecodePredCount_2)(pat, esize); let operand1 : bits(VL) = Z{}(dn); var result : bits(VL); for e = 0 to elements-1 do result[e\*:esize] = operand1[e\*:esize] - (count \* imm); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;
