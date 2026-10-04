## MOV (to/from SP)

Move register value to or from SP

This instruction copies the value of a register to or from the stack pointer.

This is an alias of [ADD (immediate)](add_addsub_imm.html). This means:

- The encodings in this description are named to match the encodings of [ADD (immediate)](add_addsub_imm.html).
- The description of [ADD (immediate)](add_addsub_imm.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

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
<td class="lr">0</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td class="droppedname">op</td>
<td class="droppedname">S</td>
<td colspan="6"></td>
<td class="droppedname">sh</td>
<td colspan="12" class="droppedname">imm12</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="MOV_ADD_32_addsub_imm"></span> Applies when <span class="bitdiff"> (sf == 0)</span>

MOV [\<Wd\|WSP\>](#WdWSP_option "Is the 32-bit name of the destination general-purpose register or stack pointer, encoded in the "Rd" field."), [\<Wn\|WSP\>](#WnWSP_option "Is the 32-bit name of the source general-purpose register or stack pointer, encoded in the "Rn" field.")

is equivalent to

[ADD](add_addsub_imm.html#ADD_32_addsub_imm) [\<Wd\|WSP\>](add_addsub_imm.html#WdWSP_option), [\<Wn\|WSP\>](add_addsub_imm.html#WnWSP_option), \#0

and is the preferred disassembly when <span class="pseudocode">Rd == '11111' \|\| Rn == '11111'</span>.

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="MOV_ADD_64_addsub_imm"></span> Applies when <span class="bitdiff"> (sf == 1)</span>

MOV [\<Xd\|SP\>](#XdSP_option "Is the 64-bit name of the destination general-purpose register or stack pointer, encoded in the "Rd" field."), [\<Xn\|SP\>](#XnSP_option__3 "Is the 64-bit name of the source general-purpose register or stack pointer, encoded in the "Rn" field.")

is equivalent to

[ADD](add_addsub_imm.html#ADD_64_addsub_imm) [\<Xd\|SP\>](add_addsub_imm.html#XdSP_option), [\<Xn\|SP\>](add_addsub_imm.html#XnSP_option__3), \#0

and is the preferred disassembly when <span class="pseudocode">Rd == '11111' \|\| Rn == '11111'</span>.

</div>

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wd|WSP&gt;</td>
<td><span id="WdWSP_option"></span>
<p>Is the 32-bit name of the destination general-purpose register or stack pointer, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

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
<td>&lt;Xd|SP&gt;</td>
<td><span id="XdSP_option"></span>
<p>Is the 64-bit name of the destination general-purpose register or stack pointer, encoded in the "Rd" field.</p></td>
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

The description of [ADD (immediate)](add_addsub_imm.html) gives the operational pseudocode for this instruction.

</div>

<div class="alias_ps_section">

### Operational Information

The description of [ADD (immediate)](add_addsub_imm.html) gives the operational information for this instruction.

</div>
