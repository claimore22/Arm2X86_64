## MOV (array to vector, four registers)

Move four ZA single-vector groups to Z four-vector operand

This instruction operates on four ZA single-vector groups.

The single-vector group within each quarter of the ZA array is selected by the sum of the vector select register and offset, modulo quarter the number of ZA array vectors.

The vector group symbol VGx4 indicates that the instruction operates on four ZA single-vector groups.

The preferred disassembly syntax uses a 64-bit element size, but an assembler should accept any element size if it is used consistently for all operands. The vector group symbol is preferred for disassembly, but optional in assembler source code.

This instruction is unpredicated.

This is an alias of [MOVA (array to vector, four registers)](mova_mz_za4.html). This means:

- The encodings in this description are named to match the encodings of [MOVA (array to vector, four registers)](mova_mz_za4.html).
- The description of [MOVA (array to vector, four registers)](mova_mz_za4.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

### <span id="iclass_sme2"></span>SME2<span style="font-size:smaller;">\
(FEAT_SME2) </span>

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
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="2" class="lr">Rv</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="3" class="lr">off3</td>
<td colspan="3" class="lr">Zd</td>
<td class="l">0</td>
<td class="r">0</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="mov_mz_za4_1_mova_mz_za4_1"></span>

MOV { [\<Zd1\>](#Zd1__2 "Is the name of the first scalable vector register of the destination multi-vector group, encoded as "Zd" times 4.").D-[\<Zd4\>](#Zd4 "Is the name of the fourth scalable vector register of the destination multi-vector group, encoded as "Zd" times 4 plus 3.").D }, ZA.D\[[\<Wv\>](#Wv "Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field."), [\<offs\>](#offs "Is the vector select offset, in the range 0 to 7, encoded in the "off3" field."){, VGx4}\]

is equivalent to

[MOVA](mova_mz_za4.html#mova_mz_za4_1) { [\<Zd1\>](mova_mz_za4.html#Zd1__2).D-[\<Zd4\>](mova_mz_za4.html#Zd4).D }, ZA.D\[[\<Wv\>](mova_mz_za4.html#Wv), [\<offs\>](mova_mz_za4.html#offs){, VGx4}\]

and is always the preferred disassembly.

</div>

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zd1&gt;</td>
<td><span id="Zd1__2"></span>
<p>Is the name of the first scalable vector register of the destination multi-vector group, encoded as "Zd" times 4.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zd4&gt;</td>
<td><span id="Zd4"></span>
<p>Is the name of the fourth scalable vector register of the destination multi-vector group, encoded as "Zd" times 4 plus 3.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Wv&gt;</td>
<td><span id="Wv"></span>
<p>Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;offs&gt;</td>
<td><span id="offs"></span>
<p>Is the vector select offset, in the range 0 to 7, encoded in the "off3" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [MOVA (array to vector, four registers)](mova_mz_za4.html) gives the operational pseudocode for this instruction.

</div>

<div class="alias_ps_section">

### Operational Information

The description of [MOVA (array to vector, four registers)](mova_mz_za4.html) gives the operational information for this instruction.

</div>
