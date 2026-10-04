## TST (immediate)

Test bits (immediate)

This instruction performs a bitwise AND between a register value and an immediate value, and discards the results. It updates the condition flags based on the result.

This is an alias of [ANDS (immediate)](ands_log_imm.html). This means:

- The encodings in this description are named to match the encodings of [ANDS (immediate)](ands_log_imm.html).
- The description of [ANDS (immediate)](ands_log_imm.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

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
<td class="l">1</td>
<td class="r">1</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">N</td>
<td colspan="6" class="lr">immr</td>
<td colspan="6" class="lr">imms</td>
<td colspan="5" class="lr">Rn</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td colspan="6"></td>
<td></td>
<td colspan="6"></td>
<td colspan="6"></td>
<td colspan="5"></td>
<td colspan="5" class="droppedname">Rd</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="TST_ANDS_32S_log_imm"></span> Applies when <span class="bitdiff"> (sf == 0 && N == 0)</span>

TST [\<Wn\>](#WnOrWZR "Is the 32-bit name of the general-purpose source register, encoded in the "Rn" field."), \#[\<imm\>](#imm__bitmask "Is the bitmask immediate. For encoding information, see x[Logical (immediate)](CHDBGEIB).")

is equivalent to

[ANDS](ands_log_imm.html#ANDS_32S_log_imm) WZR, [\<Wn\>](ands_log_imm.html#WnOrWZR), \#[\<imm\>](ands_log_imm.html#imm__bitmask)

and is always the preferred disassembly.

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="TST_ANDS_64S_log_imm"></span> Applies when <span class="bitdiff"> (sf == 1)</span>

TST [\<Xn\>](#XnOrXZR__11 "Is the 64-bit name of the general-purpose source register, encoded in the "Rn" field."), \#[\<imm\>](#imm__bitmask "Is the bitmask immediate. For encoding information, see x[Logical (immediate)](CHDBGEIB).")

is equivalent to

[ANDS](ands_log_imm.html#ANDS_64S_log_imm) XZR, [\<Xn\>](ands_log_imm.html#XnOrXZR__11), \#[\<imm\>](ands_log_imm.html#imm__bitmask)

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
<td><span id="WnOrWZR"></span>
<p>Is the 32-bit name of the general-purpose source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm__bitmask"></span>
<p>Is the bitmask immediate. For encoding information, see <span class="armarm-xref" data-linkend="CHDBGEIB">Logical (immediate)</span>.</p></td>
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

<div class="alias_ps_section">

### Operation

The description of [ANDS (immediate)](ands_log_imm.html) gives the operational pseudocode for this instruction.

</div>

<div class="alias_ps_section">

### Operational Information

The description of [ANDS (immediate)](ands_log_imm.html) gives the operational information for this instruction.

</div>
