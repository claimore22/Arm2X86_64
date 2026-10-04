## PRFM (literal)

Prefetch memory (literal)

This instruction signals the memory system that data memory accesses from a specified address are likely to occur in the near future. The address for data memory accesses is calculated from the PC value and an immediate offset. The memory system can respond by taking actions that are expected to speed up the memory accesses when they do occur, such as making the cache line containing the specified address available at the level of cache specified by the instruction.

The \<prfop\> operand specifies the prefetch hint as follows:

- Access type:
  - PLD for prefetch for load.
  - PLI for prefetch for execute.
  - PST for prefetch for store.
- Target cache level:
  - L1 for Level 1 cache.
  - L2 for Level 2 cache.
  - L3 for Level 3 cache.
  - When FEAT_PRFMSLC is implemented, SLC for system level cache.
- Policy:
  - KEEP for retained or temporal prefetch, allocated in the cache normally.
  - STRM for streaming or non-temporal prefetch, for data that is used only once.

The effect of a <span class="asm-code">PRFM</span> instruction is <span class="arm-defined-word">IMPLEMENTATION DEFINED</span>. For more information, see <span class="armarm-xref" data-linkend="ARMARM_CEGGGIDE">Prefetch memory</span>.

For information about addressing modes, see <span class="armarm-xref" data-linkend="ARMARM_CHDIIIBB">Load/Store addressing modes</span>.

<div class="regdiagram-32">

<table class="regdiagram">
<thead>
<tr>
<th>31</th>
<th>30</th>
<th>29</th>
<th>28</th>
<th>27</th>
<th>26</th>
<th>25</th>
<th>24</th>
<th>23</th>
<th>22</th>
<th>21</th>
<th>20</th>
<th>19</th>
<th>18</th>
<th>17</th>
<th>16</th>
<th>15</th>
<th>14</th>
<th>13</th>
<th>12</th>
<th>11</th>
<th>10</th>
<th>9</th>
<th>8</th>
<th>7</th>
<th>6</th>
<th>5</th>
<th>4</th>
<th>3</th>
<th>2</th>
<th>1</th>
<th>0</th>
</tr>
</thead>
<tbody>
<tr class="firstrow">
<td class="l">1</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="19" class="lr">imm19</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2" class="droppedname">opc</td>
<td colspan="2"></td>
<td></td>
<td class="droppedname">VR</td>
<td></td>
<td></td>
<td colspan="19"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="PRFM_P_loadlit"></span>

PRFM ([\<prfop\>](#Rt_prfop "Is the prefetch operation, ")\|#[\<imm5\>](#Rt_imm5 "Is the prefetch operation encoding as an immediate, in the range 0 to 31, encoded in the "Rt" field.  This syntax is only for encodings that are not accessible using <prfop>.")), [\<label\>](#imm19_offset__2 "Is the program label from which the data is to be loaded. Its offset from the address of this instruction, in the range +/-1MB, is encoded as "imm19" times 4.")

</div>

#### Decode for this encoding

let t : integer = UInt(Rt); let offset : bits(64) = SignExtend{}(imm19::'00');

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;prfop&gt;</td>
<td><span id="Rt_prfop"></span>
<p>Is the prefetch operation, encoded in “Rt”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">Rt</th>
<th class="symbol">&lt;prfop&gt;</th>
<th class="symbol">Architectural Feature</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00000</td>
<td class="symbol">PLDL1KEEP</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">00001</td>
<td class="symbol">PLDL1STRM</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">00010</td>
<td class="symbol">PLDL2KEEP</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">00011</td>
<td class="symbol">PLDL2STRM</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">00100</td>
<td class="symbol">PLDL3KEEP</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">00101</td>
<td class="symbol">PLDL3STRM</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">00110</td>
<td class="symbol">PLDSLCKEEP</td>
<td class="feature">FEAT_PRFMSLC</td>
</tr>
<tr>
<td class="bitfield">00111</td>
<td class="symbol">PLDSLCSTRM</td>
<td class="feature">FEAT_PRFMSLC</td>
</tr>
<tr>
<td class="bitfield">01000</td>
<td class="symbol">PLIL1KEEP</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">01001</td>
<td class="symbol">PLIL1STRM</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">01010</td>
<td class="symbol">PLIL2KEEP</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">01011</td>
<td class="symbol">PLIL2STRM</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">01100</td>
<td class="symbol">PLIL3KEEP</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">01101</td>
<td class="symbol">PLIL3STRM</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">01110</td>
<td class="symbol">PLISLCKEEP</td>
<td class="feature">FEAT_PRFMSLC</td>
</tr>
<tr>
<td class="bitfield">01111</td>
<td class="symbol">PLISLCSTRM</td>
<td class="feature">FEAT_PRFMSLC</td>
</tr>
<tr>
<td class="bitfield">10000</td>
<td class="symbol">PSTL1KEEP</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">10001</td>
<td class="symbol">PSTL1STRM</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">10010</td>
<td class="symbol">PSTL2KEEP</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">10011</td>
<td class="symbol">PSTL2STRM</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">10100</td>
<td class="symbol">PSTL3KEEP</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">10101</td>
<td class="symbol">PSTL3STRM</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">10110</td>
<td class="symbol">PSTSLCKEEP</td>
<td class="feature">FEAT_PRFMSLC</td>
</tr>
<tr>
<td class="bitfield">10111</td>
<td class="symbol">PSTSLCSTRM</td>
<td class="feature">FEAT_PRFMSLC</td>
</tr>
</tbody>
</table>
For other encodings of the "Rt" field, use &lt;imm5&gt;.</td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm5&gt;</td>
<td><span id="Rt_imm5"></span>
<p>Is the prefetch operation encoding as an immediate, in the range 0 to 31, encoded in the "Rt" field.</p>
<p>This syntax is only for encodings that are not accessible using &lt;prfop&gt;.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;label&gt;</td>
<td><span id="imm19_offset__2"></span>
<p>Is the program label from which the data is to be loaded. Its offset from the address of this instruction, in the range +/-1MB, is encoded as "imm19" times 4.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

let address : bits(64) = [PC64](shared_pseudocode.html#func_PC64_0)() + offset; let supports_ir : boolean = FALSE; [Prefetch](shared_pseudocode.html#func_Prefetch_3)(address, t\[4:0\], supports_ir);

</div>
