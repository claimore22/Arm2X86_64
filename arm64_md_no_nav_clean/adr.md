## ADR

Form PC-relative address This instruction adds an immediate value to the PC value to form a PC-relative address, and writes the result to the destination register.

#### Decode

let d : integer{} = UInt(Rd); let imm : bits(64) = SignExtend{}(immhi::immlo);

### Operation

[X](shared_pseudocode.html#accessor_X_2){64}(d) = [PC64](shared_pseudocode.html#func_PC64_0)() + imm;
