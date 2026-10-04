## CMN (shifted register)

Compare negative (shifted register)

This instruction adds a register value and an optionally-shifted register value. It updates the condition flags based on the result, and discards the result.

This is an alias of [ADDS (shifted register)](adds_addsub_shift.html). This means:

- The encodings in this description are named to match the encodings of [ADDS (shifted register)](adds_addsub_shift.html).
- The description of [ADDS (shifted register)](adds_addsub_shift.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

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
<td class="lr">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td class="r">1</td>
<td colspan="2" class="lr">shift</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rm</td>
<td colspan="6" class="lr">imm6</td>
<td colspan="5" class="lr">Rn</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
</tr>
<tr class="secondrow">
<td></td>
<td class="droppedname">op</td>
<td class="droppedname">S</td>
<td colspan="5"></td>
<td colspan="2"></td>
<td></td>
<td colspan="5"></td>
<td colspan="6"></td>
<td colspan="5"></td>
<td colspan="5" class="droppedname">Rd</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="CMN_ADDS_32_addsub_shift"></span> Applies when <span class="bitdiff"> (sf == 0)</span>

CMN [\<Wn\>](#WnOrWZR__3 "Is the 32-bit name of the first general-purpose source register, encoded in the "Rn" field."), [\<Wm\>](#WmOrWZR__2 "Is the 32-bit name of the second general-purpose source register, encoded in the "Rm" field."){, [\<shift\>](#shift_option__2 "Is the optional shift type to be applied to the second source operand, defaulting to LSL and ") \#[\<amount\>](#amount__5 "For the "32-bit" variant: is the shift amount, in the range 0 to 31, defaulting to 0 and encoded in the "imm6" field.")}

is equivalent to

[ADDS](adds_addsub_shift.html#ADDS_32_addsub_shift) WZR, [\<Wn\>](adds_addsub_shift.html#WnOrWZR__3), [\<Wm\>](adds_addsub_shift.html#WmOrWZR__2){, [\<shift\>](adds_addsub_shift.html#shift_option__2) \#[\<amount\>](adds_addsub_shift.html#amount__5)}

and is always the preferred disassembly.

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="CMN_ADDS_64_addsub_shift"></span> Applies when <span class="bitdiff"> (sf == 1)</span>

CMN [\<Xn\>](#XnOrXZR__12 "Is the 64-bit name of the first general-purpose source register, encoded in the "Rn" field."), [\<Xm\>](#XmOrXZR__4 "Is the 64-bit name of the second general-purpose source register, encoded in the "Rm" field."){, [\<shift\>](#shift_option__2 "Is the optional shift type to be applied to the second source operand, defaulting to LSL and ") \#[\<amount\>](#amount__6 "For the "64-bit" variant: is the shift amount, in the range 0 to 63, defaulting to 0 and encoded in the "imm6" field.")}

is equivalent to

[ADDS](adds_addsub_shift.html#ADDS_64_addsub_shift) XZR, [\<Xn\>](adds_addsub_shift.html#XnOrXZR__12), [\<Xm\>](adds_addsub_shift.html#XmOrXZR__4){, [\<shift\>](adds_addsub_shift.html#shift_option__2) \#[\<amount\>](adds_addsub_shift.html#amount__6)}

and is always the preferred disassembly.

</div>

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wn&gt;</td>
<td><span id="WnOrWZR__3"></span>
<p>Is the 32-bit name of the first general-purpose source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Wm&gt;</td>
<td><span id="WmOrWZR__2"></span>
<p>Is the 32-bit name of the second general-purpose source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;shift&gt;</td>
<td><span id="shift_option__2"></span>
<p>Is the optional shift type to be applied to the second source operand, defaulting to LSL and encoded in “shift”:</p>
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
<td class="symbol">RESERVED</td>
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
<td>&lt;Xn&gt;</td>
<td><span id="XnOrXZR__12"></span>
<p>Is the 64-bit name of the first general-purpose source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xm&gt;</td>
<td><span id="XmOrXZR__4"></span>
<p>Is the 64-bit name of the second general-purpose source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [ADDS (shifted register)](adds_addsub_shift.html) gives the operational pseudocode for this instruction.

</div>

<div class="alias_ps_section">

### Operational Information

The description of [ADDS (shifted register)](adds_addsub_shift.html) gives the operational information for this instruction.

</div>
