## CBHLT

Compare signed less than halfwords and branch

This instruction compares the signed halfword values in two registers, and conditionally branches to a label at a PC-relative offset if the second value is less than the first. This instruction provides a hint that this is not a subroutine call or return. This instruction does not affect the condition flags.

This is a pseudo-instruction of [CBH\<cc\>](cbhcc_regs.html). This means:

- The encodings in this description are named to match the encodings of [CBH\<cc\>](cbhcc_regs.html).
- The assembler syntax is used only for assembly, and is not used on disassembly.
- The description of [CBH\<cc\>](cbhcc_regs.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

### <span id="iclass_branch"></span>Branch<span style="font-size:smaller;">\
(FEAT_CMPBR) </span>

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
<td>1</td>
<td class="r">1</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rm</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td colspan="9" class="lr">imm9</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="3" class="droppedname">cc</td>
<td colspan="5"></td>
<td></td>
<td class="droppedname">H</td>
<td colspan="9"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="CBHLT_CBHGT_16_regs"></span>

CBHLT [\<Wm\>](#WmOrWZR__2 "Is the 32-bit name of the second general-purpose source register, encoded in the "Rm" field."), [\<Wt\>](#WtOrWZR "Is the 32-bit name of the general-purpose register to be tested, encoded in the "Rt" field."), [\<label\>](#imm9_offset "Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range -1024 to 1020, is encoded as "imm9" times 4.")

is equivalent to

[CBHGT](cbhcc_regs.html#CBHGT_16_regs) [\<Wt\>](cbhcc_regs.html#WtOrWZR), [\<Wm\>](cbhcc_regs.html#WmOrWZR__2), [\<label\>](cbhcc_regs.html#imm9_offset)

</div>

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

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
<td>&lt;Wt&gt;</td>
<td><span id="WtOrWZR"></span>
<p>Is the 32-bit name of the general-purpose register to be tested, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;label&gt;</td>
<td><span id="imm9_offset"></span>
<p>Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range -1024 to 1020, is encoded as "imm9" times 4.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [CBH\<cc\>](cbhcc_regs.html) gives the operational pseudocode for this instruction.

</div>
