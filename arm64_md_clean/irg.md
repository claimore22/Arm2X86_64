## IRG

Insert random tag This instruction inserts a random Logical Address Tag into the address in the first source register, and writes the result to the destination register. Any tags specified in the optional second source register or in GCR_EL1.Exclude are excluded from the selection of the random Logical Address Tag.

#### Decode

if !IsFeatureImplemented(FEAT_MTE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm);

### Operation

let operand : bits(64) = if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() else [X](shared_pseudocode.html#accessor_X_2){64}(n); let exclude_reg : bits(64) = X{}(m); let exclude : bits(16) = exclude_reg[15:0] OR GCR_EL1().Exclude; let rtag : bits(4) = [AArch64_ChooseTagOrZero](shared_pseudocode.html#func_AArch64_ChooseTagOrZero_1)(exclude); let result : bits(64) = [AArch64_AddressWithAllocationTag](shared_pseudocode.html#func_AArch64_AddressWithAllocationTag_2)(operand, rtag); if d == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = result; else [X](shared_pseudocode.html#accessor_X_2){64}(d) = result; end;
