## SEVL

Send event local This instruction is a hint instruction that causes an event to be signaled locally without requiring the event to be signaled to other PEs in the multiprocessor system. It can prime a wait-loop that starts with a WFE instruction.

#### Decode

// Empty.

### Operation

[SendEventLocal](shared_pseudocode.html#func_SendEventLocal_0)();
