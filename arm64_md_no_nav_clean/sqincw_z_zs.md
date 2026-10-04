## SQINCW (vector)

Signed saturating increment vector by multiple of 32-bit predicate constraint element count This instruction determines the number of active 32-bit elements implied by the named predicate constraint, multiplies that by an immediate in the range 1 to 16 inclusive, and then uses the result to increment all destination vector elements. The results are saturated to the 32-bit signed integer range. The named predicate constraint limits the number of active elements in a single predicate to: - A fixed number (VL1 to VL256) - The largest power of two (POW2) - The largest multiple of three or four (MUL3 or MUL4) - All available, implicitly a multiple of two (ALL). Unspecified or out of range constraint encodings generate an empty predicate or zero element count rather than Undefined Instruction exception.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let dn : integer = UInt(Zdn); let pat : bits(5) = pattern; let imm : integer = UInt(imm4) + 1; let unsigned : boolean = FALSE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let count : integer = [DecodePredCount](shared_pseudocode.html#func_DecodePredCount_2)(pat, esize); let operand1 : bits(VL) = Z{}(dn); var result : bits(VL); for e = 0 to elements-1 do let op1elt : bits(esize) = operand1[e\*:esize]; if unsigned then (result[e\*:esize], -) = [UnsignedSatQ](shared_pseudocode.html#func_UnsignedSatQ_2){esize}(UInt(op1elt) + (count \* imm)); else (result[e\*:esize], -) = [SignedSatQ](shared_pseudocode.html#func_SignedSatQ_2){esize}(SInt(op1elt) + (count \* imm)); end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;
