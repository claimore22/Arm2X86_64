## CSNEG

Conditional select negation This instruction returns, in the destination register, the value of the first source register if the condition is TRUE, and otherwise returns the negated value of the second source register. This instruction is used by the alias [CNEG](cneg_csneg.html "Conditional negate").

#### Decode

let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 32 << UInt(sf); let condition : bits(4) = cond;

### Operation

var result : bits(datasize); if [ConditionHolds](shared_pseudocode.html#func_ConditionHolds_1)(condition) then result = [X](shared_pseudocode.html#accessor_X_2){datasize}(n); else result = NOT([X](shared_pseudocode.html#accessor_X_2){datasize}(m)) + 1; end; [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result;
