## ADDG

Add with tag This instruction adds an immediate value scaled by the Tag Granule to the address in the source register, modifies the Logical Address Tag of the address using an immediate value, and writes the result to the destination register. Tags specified in GCR_EL1.Exclude are excluded from the possible outputs when modifying the Logical Address Tag.

#### Decode

if !IsFeatureImplemented(FEAT_MTE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let tag_offset : bits(4) = imm4; let offset : bits(64) = LSL(ZeroExtend{64}(imm6), [LOG2_TAG_GRANULE](shared_pseudocode.html#global_LOG2_TAG_GRANULE));

### Operation

let operand1 : bits(64) = if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() else [X](shared_pseudocode.html#accessor_X_2){64}(n); let start_tag : bits(4) = [AArch64_AllocationTagFromAddress](shared_pseudocode.html#func_AArch64_AllocationTagFromAddress_1)(operand1); let exclude : bits(16) = GCR_EL1().Exclude; let rtag : bits(4) = [AArch64_ChooseNonExcludedTagOrZero](shared_pseudocode.html#func_AArch64_ChooseNonExcludedTagOrZero_3)(start_tag, tag_offset, exclude); var result : bits(64); (result, -) = [AddWithCarry](shared_pseudocode.html#func_AddWithCarry_4){64}(operand1, offset, '0'); result = [AArch64_AddressWithAllocationTag](shared_pseudocode.html#func_AArch64_AddressWithAllocationTag_2)(result, rtag); if d == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = result; else [X](shared_pseudocode.html#accessor_X_2){64}(d) = result; end;
