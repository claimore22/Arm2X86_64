## GMI

Tag mask insert This instruction inserts the tag in the first source register into the excluded set specified in the second source register, writing the new excluded set to the destination register.

#### Decode

if !IsFeatureImplemented(FEAT_MTE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm);

### Operation

let address : bits(64) = if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() else [X](shared_pseudocode.html#accessor_X_2){64}(n); var mask : bits(64) = X{}(m); let tag : bits(4) = [AArch64_AllocationTagFromAddress](shared_pseudocode.html#func_AArch64_AllocationTagFromAddress_1)(address); mask[UInt(tag)] = '1'; [X](shared_pseudocode.html#accessor_X_2){64}(d) = mask;
