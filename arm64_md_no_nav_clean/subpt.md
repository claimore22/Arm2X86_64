## SUBPT

Subtract checked pointer This instruction subtracts an optionally-shifted register value from a base address register value, and writes the result to the destination register. The optionally-shifted register value is treated as the offset. If the operation would have generated a result where the most significant 8 bits of the result register differ from the most significant 8 bits of the base register, then the result is modified such that it is likely to be non-canonical when used as an address.

#### Decode

if !IsFeatureImplemented(FEAT_CPA) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let shift : integer = UInt(imm3);

### Operation

var result : bits(64); let base : bits(64) = if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() else [X](shared_pseudocode.html#accessor_X_2){64}(n); let offset : bits(64) = LSL([X](shared_pseudocode.html#accessor_X_2){64}(m), shift); result = base - offset; result = [PointerAddCheck](shared_pseudocode.html#func_PointerAddCheck_2)(result, base); if d == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = result; else [X](shared_pseudocode.html#accessor_X_2){64}(d) = result; end;
