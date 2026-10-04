## RPRFM

Range prefetch memory

This instruction signals the memory system that data memory accesses from a specified range of addresses are likely to occur in the near future. The instruction may also signal the memory system about the likelihood of data reuse of the specified range of addresses. The memory system can respond by taking actions that are expected to speed up the memory accesses when they do occur, such as prefetching locations within the specified address ranges into one or more caches. The memory system may also exploit the data reuse hints to decide whether to retain the data in other caches upon eviction from the innermost caches or to discard it.

The effect of an <span class="asm-code">RPRFM</span> instruction is <span class="arm-defined-word">IMPLEMENTATION DEFINED</span>. For more information, see <span class="armarm-xref" data-linkend="ARMARM_CEGGGIDE">Prefetch memory</span>.

An <span class="asm-code">RPRFM</span> instruction specifies the type of accesses and range of addresses using the following parameters:

- 'Access type', encoded in Rt\<0\>, specifies:
  - PLD for prefetch for load.
  - PST for prefetch for store.
- 'Policy', encoded in Rt\<2:1\>, specifies:
  - KEEP for retained or temporal prefetch, allocated in the cache normally.
  - STRM for streaming or non-temporal prefetch, for data that is used only once. If a streaming prefetch is specified, then the 'ReuseDistance' parameter is ignored.
- 'BaseAddress', in the 64-bit base register, holds the initial block address for the accesses.
- 'ReuseDistance', in the metadata register bits\[63:60\], indicates the maximum number of bytes to be accessed by this PE before executing the next <span class="asm-code">RPRFM</span> instruction that specifies the same range. This includes the total number of bytes inside and outside of the range that will be accessed by the same PE. This parameter can be used to influence cache eviction and replacement policies, in order to retain the data in the most optimal levels of the memory hierarchy after each access. If software cannot easily determine the amount of other memory that will be accessed, these bits can be set to zero to indicate that 'ReuseDistance' is not known. Otherwise, these four bits encode decreasing powers of two in the range 512MiB (0b0001) to 32KiB (0b1111).
- 'Stride', in the metadata register bits\[59:38\], is a signed, two's complement integer encoding of the number of bytes to advance the block address after 'Length' bytes have been accessed, in the range -2MiB to +2MiB-1B. A negative value indicates that the block address is advanced in a descending direction.
- 'Count', in the metadata register bits\[37:22\], is an unsigned integer encoding of the number of blocks of data to be accessed minus 1, representing the range 1 to 65536 blocks. If 'Count' is 0, then the 'Stride' parameter is ignored and only a single block of contiguous bytes from 'BaseAddress' to ('BaseAddress' + 'Length' - 1) is described.
- 'Length', in the metadata register bits\[21:0\], is a signed, two's complement integer encoding of the number of contiguous bytes to be accessed starting from the current block address, without changing the block address, in the range -2MiB to +2MiB-1B. A negative value indicates that the bytes are accessed in a descending direction.

<div class="note">

------------------------------------------------------------------------

#### Note

Software is expected to honor the parameters it provides to the <span class="asm-code">RPRFM</span> instruction, and the same PE should access all locations in the range, in the direction specified by the sign of the 'Length' and 'Stride' parameters. A range prefetch is considered active on a PE until all locations in the range have been accessed by the PE. A range prefetch might also be inactivated by the PE prior to completion, for example due to a software context switch or lack of hardware resources.

Software should not specify overlapping addresses in multiple active ranges. If a range is expected to be accessed by both load and store instructions (read-modify-write), then a single range with an 'Access type' parameter of PST (prefetch for store) should be specified.

------------------------------------------------------------------------

</div>

### <span id="iclass_integer"></span>Integer<span style="font-size:smaller;">\
(FEAT_RPRFM) </span>

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
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rm</td>
<td class="l">x</td>
<td>1</td>
<td class="r">x</td>
<td class="lr">S</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td class="l">1</td>
<td>1</td>
<td>x</td>
<td>x</td>
<td class="r">x</td>
</tr>
<tr class="secondrow">
<td colspan="2" class="droppedname">size</td>
<td colspan="2"></td>
<td></td>
<td class="droppedname">VR</td>
<td></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="5"></td>
<td colspan="3" class="droppedname">option</td>
<td></td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5" class="droppedname">Rt</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="RPRFM_R_ldst_regoff"></span>

RPRFM ([\<rprfop\>](#RtSoption_rprfop "Is the range prefetch operation, ")\|#[\<imm6\>](#RtSoption_imm6 "Is the range prefetch operation encoding as an immediate, in the range 0 to 63, encoded in "option<2>:option<0>:S:Rt<2:0>". This syntax is only for encodings that are not representable using <rprfop>.")), [\<Xm\>](#XmOrXZR__3 "Is the 64-bit name of the general-purpose register that holds an encoding of the metadata, encoded in the "Rm" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_RPRFM) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; let operation : bits(6) = option\[2\]::option\[0\]::S::Rt\[2:0\]; let n : integer = UInt(Rn); let m : integer = UInt(Rm);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;rprfop&gt;</td>
<td><span id="RtSoption_rprfop"></span>
<p>Is the range prefetch operation, encoded in “(option :: S :: Rt)”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">option</th>
<th class="bitfield">S</th>
<th class="bitfield">Rt</th>
<th class="symbol">&lt;rprfop&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">010</td>
<td class="bitfield">0</td>
<td class="bitfield">11000</td>
<td class="symbol">PLDKEEP</td>
</tr>
<tr>
<td class="bitfield">010</td>
<td class="bitfield">0</td>
<td class="bitfield">11001</td>
<td class="symbol">PSTKEEP</td>
</tr>
<tr>
<td class="bitfield">010</td>
<td class="bitfield">0</td>
<td class="bitfield">11100</td>
<td class="symbol">PLDSTRM</td>
</tr>
<tr>
<td class="bitfield">010</td>
<td class="bitfield">0</td>
<td class="bitfield">11101</td>
<td class="symbol">PSTSTRM</td>
</tr>
</tbody>
</table>
For other encodings, use &lt;imm6&gt;.</td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm6&gt;</td>
<td><span id="RtSoption_imm6"></span>
<p>Is the range prefetch operation encoding as an immediate, in the range 0 to 63, encoded in "option&lt;2&gt;:option&lt;0&gt;:S:Rt&lt;2:0&gt;". This syntax is only for encodings that are not representable using &lt;rprfop&gt;.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xm&gt;</td>
<td><span id="XmOrXZR__3"></span>
<p>Is the 64-bit name of the general-purpose register that holds an encoding of the metadata, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xn|SP&gt;</td>
<td><span id="XnSP_option"></span>
<p>Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

let address : bits(64) = if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() else [X](shared_pseudocode.html#accessor_X_2){64}(n); let metadata : bits(64) = X{}(m); let stride : integer = SInt(metadata\[59:38\]); let count : integer = UInt(metadata\[37:22\]) + 1; let length : integer = SInt(metadata\[21:0\]); var reuse : integer; if metadata\[63:60\] == '0000' then reuse = -1; // Not known else reuse = 32768 \<\< (15 - UInt(metadata\[63:60\])); end; Hint_RangePrefetch(address, length, stride, count, reuse, operation);

</div>
