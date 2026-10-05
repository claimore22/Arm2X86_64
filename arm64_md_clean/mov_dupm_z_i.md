## MOV

Move logical bitmask immediate to vector (unpredicated) This instruction unconditionally broadcasts the logical bitmask immediate into each element of the destination vector. This instruction is unpredicated. The immediate is a 64-bit value consisting of a single run of ones or zeros repeating every 2, 4, 8, 16, 32 or 64 bits. This is an alias of [DUPM](dupm_z_i.html). This means: - The encodings in this description are named to match the encodings of [DUPM](dupm_z_i.html). - The description of [DUPM](dupm_z_i.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [DUPM](dupm_z_i.html) gives the operational pseudocode for this instruction.
