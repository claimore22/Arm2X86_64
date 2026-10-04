## SUB (immediate)

Subtract immediate value This instruction subtracts an optionally-shifted immediate value from a register value, and writes the result to the destination register.

#### Decode

let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let datasize : integer{} = 32 << UInt(sf); let imm : bits(24) = if sh == '0' then Zeros{12}::imm12 else imm12::Zeros{12};

### Operation

let operand1 : bits(datasize) = if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){datasize}() else [X](shared_pseudocode.html#accessor_X_2){datasize}(n); let operand2 : bits(datasize) = ZeroExtend{}(imm); var result : bits(datasize); (result, -) = [AddWithCarry](shared_pseudocode.html#func_AddWithCarry_4){datasize}(operand1, NOT(operand2), '1'); if d == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = ZeroExtend{64}(result); else [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result; end;
