## ORR (vector, immediate)

Bitwise inclusive OR (vector, immediate) This instruction reads each vector element from the destination SIMD&FP register, performs a bitwise OR between each result and an immediate constant, places the result into a vector, and writes the vector to the destination SIMD&FP register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let rd : integer = UInt(Rd); let datasize : integer{} = 64 << UInt(Q); let imm64 : bits(64) = [AdvSIMDExpandImm](shared_pseudocode.html#func_AdvSIMDExpandImm_3)(op, cmode, a::b::c::d::e::f::g::h); let imm : bits(datasize) = Replicate{}(imm64);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(datasize) = V{}(rd); [V](shared_pseudocode.html#accessor_V_2){datasize}(rd) = operand OR imm;
