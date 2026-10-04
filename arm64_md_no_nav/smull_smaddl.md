## SMULL (32-bit)

Signed multiply long

This instruction multiplies two 32-bit register values, and writes the result to the 64-bit destination register.

This is an alias of [SMADDL](smaddl.html). This means:

- The encodings in this description are named to match the encodings of [SMADDL](smaddl.html).
- The description of [SMADDL](smaddl.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

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
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="5" class="lr">Rm</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td class="droppedname">sf</td>
<td colspan="2" class="droppedname">op54</td>
<td></td>
<td colspan="3"></td>
<td></td>
<td class="droppedname">U</td>
<td colspan="2"></td>
<td colspan="5"></td>
<td class="droppedname">o0</td>
<td colspan="5" class="droppedname">Ra</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="SMULL_SMADDL_64WA_dp_3src"></span>

SMULL [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Wn\>](#WnOrWZR__5 "Is the 32-bit name of the first general-purpose source register holding the multiplicand, encoded in the "Rn" field."), [\<Wm\>](#WmOrWZR__6 "Is the 32-bit name of the second general-purpose source register holding the multiplier, encoded in the "Rm" field.")

is equivalent to

[SMADDL](smaddl.html#SMADDL_64WA_dp_3src) [\<Xd\>](smaddl.html#XdOrXZR__6), [\<Wn\>](smaddl.html#WnOrWZR__5), [\<Wm\>](smaddl.html#WmOrWZR__6), XZR

and is always the preferred disassembly.

</div>

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

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
<td>&lt;Wn&gt;</td>
<td><span id="WnOrWZR__5"></span>
<p>Is the 32-bit name of the first general-purpose source register holding the multiplicand, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Wm&gt;</td>
<td><span id="WmOrWZR__6"></span>
<p>Is the 32-bit name of the second general-purpose source register holding the multiplier, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [SMADDL](smaddl.html) gives the operational pseudocode for this instruction.

</div>

<div class="alias_ps_section">

### Operational Information

The description of [SMADDL](smaddl.html) gives the operational information for this instruction.

</div>
