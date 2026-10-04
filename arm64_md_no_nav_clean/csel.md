## CSEL

Conditional select This instruction writes the value of the first source register to the destination register if the condition is TRUE. If the condition is FALSE, it writes the value of the second source register to the destination register.

#### Decode

let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 32 << UInt(sf); let condition : bits(4) = cond;

### Operation

var result : bits(datasize); if [ConditionHolds](shared_pseudocode.html#func_ConditionHolds_1)(condition) then result = [X](shared_pseudocode.html#accessor_X_2){datasize}(n); else result = [X](shared_pseudocode.html#accessor_X_2){datasize}(m); end; [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result;
