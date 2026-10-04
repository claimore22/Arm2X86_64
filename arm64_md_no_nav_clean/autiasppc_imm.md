## AUTIASPPC

Authenticate return address using key A, using an immediate offset This instruction authenticates an instruction address, using two modifiers and key A. If the authentication passes, the upper bits of the address are restored to enable subsequent use of the address. For information on behavior if the authentication fails, see Faulting on pointer authentication. The address is in X30. The first modifier is in SP. The second modifier is the address of a program label.

#### Decode

if !IsFeatureImplemented(FEAT_PAuth_LR) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = 30; let offset : bits(64) = ZeroExtend{}(imm16::'00'); let auth_combined : boolean = FALSE;

### Operation

let pac_addr : bits(64) = [PC64](shared_pseudocode.html#func_PC64_0)() - offset; [X](shared_pseudocode.html#accessor_X_2){64}(d) = [AuthIA2](shared_pseudocode.html#func_AuthIA2_4)([X](shared_pseudocode.html#accessor_X_2){64}(d), [SP](shared_pseudocode.html#accessor_SP_1){64}(), pac_addr, auth_combined);
