## AUTDA, AUTDZA

Authenticate data address, using key A This instruction authenticates a data address, using a modifier and key A. The address is in the general-purpose register that is specified by <Xd>. The modifier is: - In the general-purpose register or stack pointer that is specified by <Xn\|SP>, for AUTDA. - The value zero, for AUTDZA. If the authentication passes, the upper bits of the address are restored to enable subsequent use of the address. For information on behavior if the authentication fails, see Faulting on pointer authentication.

#### Decode

if !IsFeatureImplemented(FEAT_PAuth) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if Z == '1' && Rn != '11111' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let auth_combined : boolean = FALSE; let d : integer = UInt(Rd); let n : integer = UInt(Rn); let source_is_sp : boolean = Z == '0' && n == 31;

### Operation

if source_is_sp then [X](shared_pseudocode.html#accessor_X_2){64}(d) = [AuthDA](shared_pseudocode.html#func_AuthDA_3)([X](shared_pseudocode.html#accessor_X_2){64}(d), [SP](shared_pseudocode.html#accessor_SP_1){64}(), auth_combined); else [X](shared_pseudocode.html#accessor_X_2){64}(d) = [AuthDA](shared_pseudocode.html#func_AuthDA_3)([X](shared_pseudocode.html#accessor_X_2){64}(d), [X](shared_pseudocode.html#accessor_X_2){64}(n), auth_combined); end;
