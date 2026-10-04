## CSINV

Conditional select invert This instruction returns, in the destination register, the value of the first source register if the condition is TRUE, and otherwise returns the bitwise inversion value of the second source register. This instruction is used by the aliases [CINV](cinv_csinv.html "Conditional invert"), and [CSETM](csetm_csinv.html "Conditional set mask").

#### Decode

let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 32 << UInt(sf); let condition : bits(4) = cond;

### Operation

var result : bits(datasize); if [ConditionHolds](shared_pseudocode.html#func_ConditionHolds_1)(condition) then result = [X](shared_pseudocode.html#accessor_X_2){datasize}(n); else result = NOT([X](shared_pseudocode.html#accessor_X_2){datasize}(m)); end; [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result;
