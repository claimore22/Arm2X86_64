## ADRP

Form PC-relative address to 4KB page This instruction adds an immediate value that is shifted left by 12 bits, to the PC value to form a PC-relative address, with the bottom 12 bits masked out, and writes the result to the destination register.

#### Decode

let d : integer{} = UInt(Rd); let imm : bits(64) = SignExtend{}(immhi::immlo::Zeros{12});

### Operation

let base : bits(64) = [PC64](shared_pseudocode.html#func_PC64_0)()[63:12]::Zeros{12}; [X](shared_pseudocode.html#accessor_X_2){64}(d) = base + imm;
