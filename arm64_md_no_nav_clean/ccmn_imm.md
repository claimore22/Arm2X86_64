## CCMN (immediate)

Conditional compare negative (immediate) This instruction sets the value of the condition flags to the result of the comparison of a register value and a negated immediate value if the condition is TRUE, and an immediate value otherwise.

#### Decode

let n : integer{} = UInt(Rn); let datasize : integer{} = 32 << UInt(sf); let condition : bits(4) = cond; var flags : bits(4) = nzcv; let imm : bits(datasize) = ZeroExtend{}(imm5);

### Operation

if [ConditionHolds](shared_pseudocode.html#func_ConditionHolds_1)(condition) then let operand1 : bits(datasize) = X{}(n); let operand2 : bits(datasize) = imm; (-, flags) = [AddWithCarry](shared_pseudocode.html#func_AddWithCarry_4){datasize}(operand1, operand2, '0'); end; PSTATE.[N,Z,C,V] = flags;
