## AND (immediate)

Bitwise AND (immediate) This instruction performs a bitwise AND between a register value and an immediate value, and writes the result to the destination register.

#### Decode

if sf == '0' && N != '0' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let datasize : integer{} = 32 << UInt(sf); var imm : bits(datasize); (imm, -) = [DecodeBitMasks](shared_pseudocode.html#func_DecodeBitMasks_5){datasize}(N, imms, immr, TRUE);

### Operation

let operand1 : bits(datasize) = X{}(n); let operand2 : bits(datasize) = imm; let result : bits(datasize) = operand1 AND operand2; if d == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = ZeroExtend{64}(result); else [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result; end;
