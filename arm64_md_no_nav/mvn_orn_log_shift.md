## MVN

Bitwise NOT

This instruction writes the bitwise inverse of a register value to the destination register.

This is an alias of [ORN (shifted register)](orn_log_shift.html). This means:

- The encodings in this description are named to match the encodings of [ORN (shifted register)](orn_log_shift.html).
- The description of [ORN (shifted register)](orn_log_shift.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

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
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td colspan="2" class="lr">shift</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rm</td>
<td colspan="6" class="lr">imm6</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td colspan="5"></td>
<td colspan="2"></td>
<td class="droppedname">N</td>
<td colspan="5"></td>
<td colspan="6"></td>
<td colspan="5" class="droppedname">Rn</td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="MVN_ORN_32_log_shift"></span> Applies when <span class="bitdiff"> (sf == 0)</span>

MVN [\<Wd\>](#WdOrWZR "Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Wm\>](#WmOrWZR__3 "Is the 32-bit name of the general-purpose source register, encoded in the "Rm" field."){, [\<shift\>](#shift_option__3 "Is the optional shift to be applied to the final source, defaulting to LSL and ") \#[\<amount\>](#amount__5 "For the "32-bit" variant: is the shift amount, in the range 0 to 31, defaulting to 0 and encoded in the "imm6" field.")}

is equivalent to

[ORN](orn_log_shift.html#ORN_32_log_shift) [\<Wd\>](orn_log_shift.html#WdOrWZR), WZR, [\<Wm\>](orn_log_shift.html#WmOrWZR__2){, [\<shift\>](orn_log_shift.html#shift_option__3) \#[\<amount\>](orn_log_shift.html#amount__5)}

and is always the preferred disassembly.

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="MVN_ORN_64_log_shift"></span> Applies when <span class="bitdiff"> (sf == 1)</span>

MVN [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Xm\>](#XmOrXZR "Is the 64-bit name of the general-purpose source register, encoded in the "Rm" field."){, [\<shift\>](#shift_option__3 "Is the optional shift to be applied to the final source, defaulting to LSL and ") \#[\<amount\>](#amount__6 "For the "64-bit" variant: is the shift amount, in the range 0 to 63, defaulting to 0 and encoded in the "imm6" field.")}

is equivalent to

[ORN](orn_log_shift.html#ORN_64_log_shift) [\<Xd\>](orn_log_shift.html#XdOrXZR__6), XZR, [\<Xm\>](orn_log_shift.html#XmOrXZR__4){, [\<shift\>](orn_log_shift.html#shift_option__3) \#[\<amount\>](orn_log_shift.html#amount__6)}

and is always the preferred disassembly.

</div>

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
<td>&lt;Wm&gt;</td>
<td><span id="WmOrWZR__3"></span>
<p>Is the 32-bit name of the general-purpose source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;shift&gt;</td>
<td><span id="shift_option__3"></span>
<p>Is the optional shift to be applied to the final source, defaulting to LSL and encoded in “shift”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">shift</th>
<th class="symbol">&lt;shift&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">LSL</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">LSR</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="symbol">ASR</td>
</tr>
<tr>
<td class="bitfield">11</td>
<td class="symbol">ROR</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;amount&gt;</td>
<td><span id="amount__5"></span>
<p>For the "32-bit" variant: is the shift amount, in the range 0 to 31, defaulting to 0 and encoded in the "imm6" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="amount__6"></span>
<p>For the "64-bit" variant: is the shift amount, in the range 0 to 63, defaulting to 0 and encoded in the "imm6" field.</p></td>
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
<td>&lt;Xm&gt;</td>
<td><span id="XmOrXZR"></span>
<p>Is the 64-bit name of the general-purpose source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [ORN (shifted register)](orn_log_shift.html) gives the operational pseudocode for this instruction.

</div>

<div class="alias_ps_section">

### Operational Information

The description of [ORN (shifted register)](orn_log_shift.html) gives the operational information for this instruction.

</div>
