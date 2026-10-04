## CCMP (register)

Conditional compare (register) This instruction sets the value of the condition flags to the result of the comparison of two registers if the condition is TRUE, and an immediate value otherwise.

#### Decode

let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 32 << UInt(sf); let condition : bits(4) = cond; var flags : bits(4) = nzcv;

### Operation

if [ConditionHolds](shared_pseudocode.html#func_ConditionHolds_1)(condition) then let operand1 : bits(datasize) = X{}(n); let operand2 : bits(datasize) = [X](shared_pseudocode.html#accessor_X_2){datasize}(m); (-, flags) = [AddWithCarry](shared_pseudocode.html#func_AddWithCarry_4){datasize}(operand1, NOT(operand2), '1'); end; PSTATE.[N,Z,C,V] = flags;
