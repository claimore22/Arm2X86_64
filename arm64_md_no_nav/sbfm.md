## SBFM

Signed bitfield move

This instruction is usually accessed via one of its aliases, which are always preferred for disassembly.

If \<imms\> is greater than or equal to \<immr\>, this copies a bitfield of (\<imms\>-\<immr\>+1) bits starting from bit position \<immr\> in the source register to the least significant bits of the destination register.

If \<imms\> is less than \<immr\>, this copies a bitfield of (\<imms\>+1) bits from the least significant bits of the source register to bit position (regsize-\<immr\>) of the destination register, where regsize is the destination register size of 32 or 64 bits.

In both cases, the destination bits below the bitfield are set to zero, and the bits above the bitfield are set to a copy of the most significant bit of the bitfield.

This instruction is used by the aliases [ASR (immediate)](asr_sbfm.html "Arithmetic shift right (immediate)"), [SBFIZ](sbfiz_sbfm.html "Signed bitfield insert in zeros"), [SBFX](sbfx_sbfm.html "Signed bitfield extract"), [SXTB](sxtb_sbfm.html "Signed extend byte"), [SXTH](sxth_sbfm.html "Sign extend halfword"), and [SXTW](sxtw_sbfm.html "Sign extend word").

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
<td class="lr">sf</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">N</td>
<td colspan="6" class="lr">immr</td>
<td colspan="6" class="lr">imms</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td colspan="6"></td>
<td></td>
<td colspan="6"></td>
<td colspan="6"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="SBFM_32M_bitfield"></span> Applies when <span class="bitdiff"> (sf == 0 && N == 0)</span>

SBFM [\<Wd\>](#WdOrWZR "Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Wn\>](#WnOrWZR "Is the 32-bit name of the general-purpose source register, encoded in the "Rn" field."), \#[\<immr\>](#immr "For the "32-bit" variant: is the right rotate amount, in the range 0 to 31, encoded in the "immr" field."), \#[\<imms\>](#imms "For the "32-bit" variant: is the leftmost bit number to be moved from the source, in the range 0 to 31, encoded in the "imms" field.")

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="SBFM_64M_bitfield"></span> Applies when <span class="bitdiff"> (sf == 1 && N == 1)</span>

SBFM [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Xn\>](#XnOrXZR__11 "Is the 64-bit name of the general-purpose source register, encoded in the "Rn" field."), \#[\<immr\>](#immr__2 "For the "64-bit" variant: is the right rotate amount, in the range 0 to 63, encoded in the "immr" field."), \#[\<imms\>](#imms__2 "For the "64-bit" variant: is the leftmost bit number to be moved from the source, in the range 0 to 63, encoded in the "imms" field.")

</div>

#### Decode for all variants of this encoding

if sf == '1' && N != '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if sf == '0' && (N != '0' \|\| immr\[5\] != '0' \|\| imms\[5\] != '0') then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let datasize : integer{} = 32 \<\< UInt(sf); let s : integer = UInt(imms); let r : integer = UInt(immr); var wmask : bits(datasize); var tmask : bits(datasize); (wmask, tmask) = [DecodeBitMasks](shared_pseudocode.html#func_DecodeBitMasks_5){datasize}(N, imms, immr, FALSE);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wd&gt;</td>
<td><span id="WdOrWZR"></span>
<p>Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Wn&gt;</td>
<td><span id="WnOrWZR"></span>
<p>Is the 32-bit name of the general-purpose source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;immr&gt;</td>
<td><span id="immr"></span>
<p>For the "32-bit" variant: is the right rotate amount, in the range 0 to 31, encoded in the "immr" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="immr__2"></span>
<p>For the "64-bit" variant: is the right rotate amount, in the range 0 to 63, encoded in the "immr" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imms&gt;</td>
<td><span id="imms"></span>
<p>For the "32-bit" variant: is the leftmost bit number to be moved from the source, in the range 0 to 31, encoded in the "imms" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="imms__2"></span>
<p>For the "64-bit" variant: is the leftmost bit number to be moved from the source, in the range 0 to 63, encoded in the "imms" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xd&gt;</td>
<td><span id="XdOrXZR__6"></span>
<p>Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xn&gt;</td>
<td><span id="XnOrXZR__11"></span>
<p>Is the 64-bit name of the general-purpose source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

### Alias Conditions

| Alias | Of variant | Is preferred when |
|----|----|----|
| [ASR (immediate)](asr_sbfm.html "Arithmetic shift right (immediate)") | 32-bit | <span class="pseudocode">imms == '011111'</span> |
| [ASR (immediate)](asr_sbfm.html "Arithmetic shift right (immediate)") | 64-bit | <span class="pseudocode">imms == '111111'</span> |
| [SBFIZ](sbfiz_sbfm.html "Signed bitfield insert in zeros") |  | <span class="pseudocode">UInt(imms) \< UInt(immr)</span> |
| [SBFX](sbfx_sbfm.html "Signed bitfield extract") |  | <span class="pseudocode">BFXPreferred(sf, opc\[1\], imms, immr)</span> |
| [SXTB](sxtb_sbfm.html "Signed extend byte") |  | <span class="pseudocode">immr == '000000' && imms == '000111'</span> |
| [SXTH](sxth_sbfm.html "Sign extend halfword") |  | <span class="pseudocode">immr == '000000' && imms == '001111'</span> |
| [SXTW](sxtw_sbfm.html "Sign extend word") |  | <span class="pseudocode">immr == '000000' && imms == '011111'</span> |

<div class="ps">

### Operation

let src : bits(datasize) = X{}(n); // Perform bitfield move on low bits let bot : bits(datasize) = ROR(src, r) AND wmask; let top : bits(datasize) = Replicate{}(src\[s\]); // Combine extension bits and result bits [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = (top AND NOT(tmask)) OR (bot AND tmask);

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
