## PACDB, PACDZB

Pointer Authentication Code for data address, using key B This instruction calculates and inserts a Pointer Authentication Code for a data address, using a modifier and key B. The address is in the general-purpose register that is specified by <Xd>. The modifier is: - In the general-purpose register or stack pointer that is specified by <Xn\|SP> for PACDB. - The value zero, for PACDZB.

#### Decode

if !IsFeatureImplemented(FEAT_PAuth) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; var source_is_sp : boolean = FALSE; let d : integer = UInt(Rd); let n : integer = UInt(Rn); if Z == '0' then // PACDB if n == 31 then source_is_sp = TRUE; end; else // PACDZB if n != 31 then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; end;

### Operation

if source_is_sp then [X](shared_pseudocode.html#accessor_X_2){64}(d) = [AddPACDB](shared_pseudocode.html#func_AddPACDB_2)([X](shared_pseudocode.html#accessor_X_2){64}(d), [SP](shared_pseudocode.html#accessor_SP_1){64}()); else [X](shared_pseudocode.html#accessor_X_2){64}(d) = [AddPACDB](shared_pseudocode.html#func_AddPACDB_2)([X](shared_pseudocode.html#accessor_X_2){64}(d), [X](shared_pseudocode.html#accessor_X_2){64}(n)); end;
