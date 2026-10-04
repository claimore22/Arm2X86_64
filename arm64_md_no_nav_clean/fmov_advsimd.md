## FMOV (vector, immediate)

Floating-point move immediate (vector) This instruction copies an immediate floating-point constant into every element of the SIMD&FP destination register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. It has encodings from 2 classes: [Half-precision](#iclass_half_precision) and [Single-precision and double-precision](#iclass_single_precision_and_double_precision)

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if cmode::op == '11111' then // FMOV Dn,#imm is in main FP instruction set if Q == '0' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; end; let rd : integer = UInt(Rd); let datasize : integer{} = 64 << UInt(Q); let imm64 : bits(64) = [AdvSIMDExpandImm](shared_pseudocode.html#func_AdvSIMDExpandImm_3)(op, cmode, a::b::c::d::e::f::g::h); let imm : bits(datasize) = Replicate{}(imm64);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); [V](shared_pseudocode.html#accessor_V_2){datasize}(rd) = imm;
