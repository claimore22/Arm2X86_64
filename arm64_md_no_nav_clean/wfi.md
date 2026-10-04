## WFI

Wait for interrupt This instruction is a hint instruction that indicates that the PE can enter a low-power state and remain there until a wakeup event occurs. For more information, see Wait For Interrupt. As described in Wait For Interrupt, the execution of a WFI instruction that would otherwise cause entry to a low-power state can be trapped to a higher Exception level.

#### Decode

// Empty.

### Operation

[Hint_WFI](shared_pseudocode.html#func_Hint_WFI_0)();
