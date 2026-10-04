## SQINCW (scalar)

Signed saturating increment scalar by multiple of 32-bit predicate constraint element count This instruction determines the number of active 32-bit elements implied by the named predicate constraint, multiplies that by an immediate in the range 1 to 16 inclusive, and then uses the result to increment the scalar destination. The result is saturated to the source general-purpose register's signed integer range. A 32-bit saturated result is then sign-extended to 64 bits. The named predicate constraint limits the number of active elements in a single predicate to: - A fixed number (VL1 to VL256) - The largest power of two (POW2) - The largest multiple of three or four (MUL3 or MUL4) - All available, implicitly a multiple of two (ALL). Unspecified or out of range constraint encodings generate an empty predicate or zero element count rather than Undefined Instruction exception. It has encodings from 2 classes: [32-bit](#iclass_32_bit) and [64-bit](#iclass_64_bit)

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let dn : integer = UInt(Rdn); let pat : bits(5) = pattern; let imm : integer = UInt(imm4) + 1; let unsigned : boolean = FALSE; let ssize : integer{} = 64;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let count : integer = [DecodePredCount](shared_pseudocode.html#func_DecodePredCount_2)(pat, esize); let operand1 : bits(ssize) = X{}(dn); var result : bits(ssize); if unsigned then (result, -) = [UnsignedSatQ](shared_pseudocode.html#func_UnsignedSatQ_2){ssize}(UInt(operand1) + (count \* imm)); else (result, -) = [SignedSatQ](shared_pseudocode.html#func_SignedSatQ_2){ssize}(SInt(operand1) + (count \* imm)); end; [X](shared_pseudocode.html#accessor_X_2){64}(dn) = Extend{64}(result, unsigned);
