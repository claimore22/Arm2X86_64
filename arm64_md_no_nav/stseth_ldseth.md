## STSETH, STSETLH

Atomic bit set on halfword, without return

This instruction atomically loads a 16-bit halfword from memory, performs a bitwise OR with the value held in a register on it, and stores the result back to memory.

- <span class="asm-code">STSETH</span> does not have release semantics.
- <span class="asm-code">STSETLH</span> stores to memory with release semantics, as described in <span class="armarm-xref" data-linkend="ARMARM_BEIHCHEF">Load-Acquire, Store-Release</span>.

For information about addressing modes, see <span class="armarm-xref" data-linkend="ARMARM_CHDIIIBB">Load/Store addressing modes</span>.

This is an alias of [LDSETH, LDSETAH, LDSETALH, LDSETLH](ldseth.html). This means:

- The encodings in this description are named to match the encodings of [LDSETH, LDSETAH, LDSETALH, LDSETLH](ldseth.html).
- The description of [LDSETH, LDSETAH, LDSETALH, LDSETLH](ldseth.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

### <span id="iclass_integer"></span>Integer<span style="font-size:smaller;">\
(FEAT_LSE) </span>

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
<td class="l">0</td>
<td class="r">1</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">R</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rs</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
</tr>
<tr class="secondrow">
<td colspan="2" class="droppedname">size</td>
<td colspan="3"></td>
<td class="droppedname">VR</td>
<td colspan="2"></td>
<td class="droppedname">A</td>
<td></td>
<td></td>
<td colspan="5"></td>
<td class="droppedname">o3</td>
<td colspan="3" class="droppedname">opc</td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5" class="droppedname">Rt</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the No memory ordering variant

<span id="STSETH_LDSETH_32_memop"></span> Applies when <span class="bitdiff"> (R == 0)</span>

STSETH [\<Ws\>](#WsOrWZR "Is the 32-bit name of the general-purpose register holding the data value to be operated on with the contents of the memory location, encoded in the "Rs" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

is equivalent to

[LDSETH](ldseth.html#LDSETH_32_memop) [\<Ws\>](ldseth.html#WsOrWZR), WZR, \[[\<Xn\|SP\>](ldseth.html#XnSP_option)\]

and is always the preferred disassembly.

</div>

<div class="encoding">

#### Encoding for the Release variant

<span id="STSETLH_LDSETLH_32_memop"></span> Applies when <span class="bitdiff"> (R == 1)</span>

STSETLH [\<Ws\>](#WsOrWZR "Is the 32-bit name of the general-purpose register holding the data value to be operated on with the contents of the memory location, encoded in the "Rs" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

is equivalent to

[LDSETLH](ldseth.html#LDSETLH_32_memop) [\<Ws\>](ldseth.html#WsOrWZR), WZR, \[[\<Xn\|SP\>](ldseth.html#XnSP_option)\]

and is always the preferred disassembly.

</div>

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Ws&gt;</td>
<td><span id="WsOrWZR"></span>
<p>Is the 32-bit name of the general-purpose register holding the data value to be operated on with the contents of the memory location, encoded in the "Rs" field.</p></td>
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

<div class="alias_ps_section">

### Operation

The description of [LDSETH, LDSETAH, LDSETALH, LDSETLH](ldseth.html) gives the operational pseudocode for this instruction.

</div>

<div class="alias_ps_section">

### Operational Information

The description of [LDSETH, LDSETAH, LDSETALH, LDSETLH](ldseth.html) gives the operational information for this instruction.

</div>
