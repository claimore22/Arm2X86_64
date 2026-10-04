## AUTIASPPCR

Authenticate return address using key A, using a register This instruction authenticates an instruction address, using two modifiers and key A. If the authentication passes, the upper bits of the address are restored to enable subsequent use of the address. For information on behavior if the authentication fails, see Faulting on pointer authentication. The address is in X30. The first modifier is in SP. The second modifier is in the general-purpose register that is specified by <Xn>.

#### Decode

if !IsFeatureImplemented(FEAT_PAuth_LR) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = 30; let n : integer = UInt(Rn); let auth_combined : boolean = FALSE;

### Operation

[X](shared_pseudocode.html#accessor_X_2){64}(d) = [AuthIA2](shared_pseudocode.html#func_AuthIA2_4)([X](shared_pseudocode.html#accessor_X_2){64}(d), [SP](shared_pseudocode.html#accessor_SP_1){64}(), [X](shared_pseudocode.html#accessor_X_2){64}(n), auth_combined);
