## DMB

Data memory barrier This instruction is a memory barrier that ensures the ordering of observations of memory accesses, see Data Memory Barrier.

#### Decode

var types : [MBReqTypes](shared_pseudocode.html#type_MBReqTypes); case CRm[1:0] of when '00' => types = [MBReqTypes_All](shared_pseudocode.html#enum_MBReqTypes_All); when '01' => types = [MBReqTypes_Reads](shared_pseudocode.html#enum_MBReqTypes_Reads); when '10' => types = [MBReqTypes_Writes](shared_pseudocode.html#enum_MBReqTypes_Writes); when '11' => types = [MBReqTypes_All](shared_pseudocode.html#enum_MBReqTypes_All); end;

### Operation

DataMemoryBarrier(types);
