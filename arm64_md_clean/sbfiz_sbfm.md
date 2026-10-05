## SBFIZ

Signed bitfield insert in zeros This instruction copies a bitfield of <width> bits from the least significant bits of the source register to bit position <lsb> of the destination register, setting the destination bits below the bitfield to zero, and the bits above the bitfield to a copy of the most significant bit of the bitfield. This is an alias of [SBFM](sbfm.html). This means: - The encodings in this description are named to match the encodings of [SBFM](sbfm.html). - The description of [SBFM](sbfm.html) gives the operational pseudocode, any constrained unpredictable behavior, and any operational information for this instruction.

### Operation

The description of [SBFM](sbfm.html) gives the operational pseudocode for this instruction.
