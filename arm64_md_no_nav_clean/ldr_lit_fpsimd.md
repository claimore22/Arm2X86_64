## LDR (literal, SIMD&FP)

Load SIMD&FP register (PC-relative literal) This instruction loads a SIMD&FP register from memory. The address that is used for the load is calculated from the PC value and an immediate offset. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_FP) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer{} = UInt(Rt); if opc == '11' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let size : integer{} = 4 << (UInt(opc) as integer{0..2}); let nontemporal : boolean = FALSE; let tagchecked : boolean = FALSE; let offset : bits(64) = SignExtend{}(imm19::'00');

### Operation

let address : bits(64) = [PC64](shared_pseudocode.html#func_PC64_0)() + offset; [AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescASIMD](shared_pseudocode.html#func_CreateAccDescASIMD_4)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), nontemporal, tagchecked, privileged); let data : bits(size\*8) = [Mem](shared_pseudocode.html#accessor_Mem_3){size\*8}(address, accdesc); [V](shared_pseudocode.html#accessor_V_2){size\*8}(t) = data;
