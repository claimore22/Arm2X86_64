## WFIT

Wait for interrupt with timeout This instruction provides a hint that the PE can enter a low-power state and remain there until either a local timeout event or a wakeup event occurs. For more information, see Wait For Interrupt. As described in Wait For Interrupt, the execution of a WFIT instruction that would otherwise cause entry to a low-power state can be trapped to a higher Exception level.

#### Decode

if !IsFeatureImplemented(FEAT_WFxT) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = UInt(Rd);

### Operation

let localtimeout : integer = UInt([X](shared_pseudocode.html#accessor_X_2){64}(d)); if [Halted](shared_pseudocode.html#func_Halted_0)() && [ConstrainUnpredictableBool](shared_pseudocode.html#func_ConstrainUnpredictableBool_1)([Unpredictable_WFxTDEBUG](shared_pseudocode.html#enum_Unpredictable_WFxTDEBUG)) then [ExecuteAsNOP](shared_pseudocode.html#func_ExecuteAsNOP_0)(); end; [Hint_WFIT](shared_pseudocode.html#func_Hint_WFIT_1)(localtimeout);
