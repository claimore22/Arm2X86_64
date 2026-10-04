## STSHH

Store shared hint This instruction signals to the memory system that if the next instruction in program order generates an explicit write effect, then it is to a location that one or more other threads of execution will observe, and there is a performance benefit to ensuring that the updated value from the write to that location propagates to those other observers with minimal latency. The thread of execution on the other observers might be polling the location using load or Load-Exclusive instructions, or may have executed a PRFM IR instruction targeting the location.

#### Decode

if !IsFeatureImplemented(FEAT_PCDPHINT) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; let stream : boolean = op2[0] == '1';

### Operation

Hint_StoreShared(stream);
