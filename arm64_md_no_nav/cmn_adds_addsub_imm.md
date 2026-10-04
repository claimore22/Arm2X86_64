## CMN (immediate)

Compare negative (immediate)

This instruction adds a register value and an optionally-shifted immediate value. It updates the condition flags based on the result, and discards the result.

This is an alias of [ADDS (immediate)](adds_addsub_imm.html). This means:

- The encodings in this description are named to match the encodings of [ADDS (immediate)](adds_addsub_imm.html).
- The description of [ADDS (immediate)](adds_addsub_imm.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

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
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">sh</td>
<td colspan="12" class="lr">imm12</td>
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
<td colspan="6"></td>
<td></td>
<td colspan="12"></td>
<td colspan="5"></td>
<td colspan="5" class="droppedname">Rd</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="CMN_ADDS_32S_addsub_imm"></span> Applies when <span class="bitdiff"> (sf == 0)</span>

CMN [\<Wn\|WSP\>](#WnWSP_option "Is the 32-bit name of the source general-purpose register or stack pointer, encoded in the "Rn" field."), \#[\<imm\>](#imm__17 "Is an unsigned immediate, in the range 0 to 4095, encoded in the "imm12" field."){, [\<shift\>](#shift_option "Is the optional left shift to apply to the immediate, defaulting to LSL #0 and ")}

is equivalent to

[ADDS](adds_addsub_imm.html#ADDS_32S_addsub_imm) WZR, [\<Wn\|WSP\>](adds_addsub_imm.html#WnWSP_option), \#[\<imm\>](adds_addsub_imm.html#imm__17){, [\<shift\>](adds_addsub_imm.html#shift_option)}

and is always the preferred disassembly.

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="CMN_ADDS_64S_addsub_imm"></span> Applies when <span class="bitdiff"> (sf == 1)</span>

CMN [\<Xn\|SP\>](#XnSP_option__3 "Is the 64-bit name of the source general-purpose register or stack pointer, encoded in the "Rn" field."), \#[\<imm\>](#imm__17 "Is an unsigned immediate, in the range 0 to 4095, encoded in the "imm12" field."){, [\<shift\>](#shift_option "Is the optional left shift to apply to the immediate, defaulting to LSL #0 and ")}

is equivalent to

[ADDS](adds_addsub_imm.html#ADDS_64S_addsub_imm) XZR, [\<Xn\|SP\>](adds_addsub_imm.html#XnSP_option__3), \#[\<imm\>](adds_addsub_imm.html#imm__17){, [\<shift\>](adds_addsub_imm.html#shift_option)}

and is always the preferred disassembly.

</div>

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wn|WSP&gt;</td>
<td><span id="WnWSP_option"></span>
<p>Is the 32-bit name of the source general-purpose register or stack pointer, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm__17"></span>
<p>Is an unsigned immediate, in the range 0 to 4095, encoded in the "imm12" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;shift&gt;</td>
<td><span id="shift_option"></span>
<p>Is the optional left shift to apply to the immediate, defaulting to LSL #0 and encoded in “sh”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">sh</th>
<th class="symbol">&lt;shift&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">LSL #0</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">LSL #12</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xn|SP&gt;</td>
<td><span id="XnSP_option__3"></span>
<p>Is the 64-bit name of the source general-purpose register or stack pointer, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [ADDS (immediate)](adds_addsub_imm.html) gives the operational pseudocode for this instruction.

</div>

<div class="alias_ps_section">

### Operational Information

The description of [ADDS (immediate)](adds_addsub_imm.html) gives the operational information for this instruction.

</div>
