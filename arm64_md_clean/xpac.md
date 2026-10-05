## XPACD, XPACI, XPACLRI

Strip Pointer Authentication Code This instruction removes the Pointer Authentication Code from an address. The address is in the specified general-purpose register for XPACI and XPACD, and is in LR for XPACLRI. The XPACD instruction is used for data addresses, and XPACI and XPACLRI are used for instruction addresses. It has encodings from 2 classes: [Integer](#iclass_integer) and [System](#iclass_system)

#### Decode

if !IsFeatureImplemented(FEAT_PAuth) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; let d : integer = 30; let data : boolean = FALSE;

### Operation

[X](shared_pseudocode.html#accessor_X_2){64}(d) = [Strip](shared_pseudocode.html#func_Strip_2)([X](shared_pseudocode.html#accessor_X_2){64}(d), data);
