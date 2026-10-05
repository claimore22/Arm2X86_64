## CSINC

Conditional select increment This instruction returns, in the destination register, the value of the first source register if the condition is TRUE, and otherwise returns the value of the second source register incremented by 1. This instruction is used by the aliases [CINC](cinc_csinc.html "Conditional increment"), and [CSET](cset_csinc.html "Conditional set").

#### Decode

let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 32 << UInt(sf); let condition : bits(4) = cond;

### Operation

var result : bits(datasize); if [ConditionHolds](shared_pseudocode.html#func_ConditionHolds_1)(condition) then result = [X](shared_pseudocode.html#accessor_X_2){datasize}(n); else result = [X](shared_pseudocode.html#accessor_X_2){datasize}(m) + 1; end; [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result;
