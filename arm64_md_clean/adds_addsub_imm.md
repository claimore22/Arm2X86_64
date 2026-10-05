## ADDS (immediate)

Add immediate value, setting flags This instruction adds a register value and an optionally-shifted immediate value, and writes the result to the destination register. It updates the condition flags based on the result. This instruction is used by the alias [CMN (immediate)](cmn_adds_addsub_imm.html "Compare negative (immediate)").

#### Decode

let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let datasize : integer{} = 32 << UInt(sf); let imm : bits(24) = if sh == '0' then Zeros{12}::imm12 else imm12::Zeros{12};

### Operation

let operand1 : bits(datasize) = if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){datasize}() else [X](shared_pseudocode.html#accessor_X_2){datasize}(n); let operand2 : bits(datasize) = ZeroExtend{}(imm); var result : bits(datasize); var nzcv : bits(4); (result, nzcv) = [AddWithCarry](shared_pseudocode.html#func_AddWithCarry_4){datasize}(operand1, operand2, '0'); [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result; PSTATE.[N,Z,C,V] = nzcv;
