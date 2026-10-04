## PRFM (literal)

Prefetch memory (literal) This instruction signals the memory system that data memory accesses from a specified address are likely to occur in the near future. The address for data memory accesses is calculated from the PC value and an immediate offset. The memory system can respond by taking actions that are expected to speed up the memory accesses when they do occur, such as making the cache line containing the specified address available at the level of cache specified by the instruction. The \ operand specifies the prefetch hint as follows: - Access type: - PLD for prefetch for load. - PLI for prefetch for execute. - PST for prefetch for store. - Target cache level: - L1 for Level 1 cache. - L2 for Level 2 cache. - L3 for Level 3 cache. - When FEAT_PRFMSLC is implemented, SLC for system level cache. - Policy: - KEEP for retained or temporal prefetch, allocated in the cache normally. - STRM for streaming or non-temporal prefetch, for data that is used only once. The effect of a PRFM instruction is IMPLEMENTATION DEFINED. For more information, see Prefetch memory. For information about addressing modes, see Load/Store addressing modes.

#### Decode

let t : integer = UInt(Rt); let offset : bits(64) = SignExtend{}(imm19::'00');

### Operation

let address : bits(64) = [PC64](shared_pseudocode.html#func_PC64_0)() + offset; let supports_ir : boolean = FALSE; [Prefetch](shared_pseudocode.html#func_Prefetch_3)(address, t[4:0], supports_ir);
