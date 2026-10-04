## BIC (immediate)

Bitwise clear bits using immediate (unpredicated)

This instruction performs a bitwise clear bits using an immediate with each 64-bit element of the source vector, and destructively places the results in the corresponding elements of the source vector. The immediate is a 64-bit value consisting of a single run of ones or zeros repeating every 2, 4, 8, 16, 32 or 64 bits. This instruction is unpredicated.

This is a pseudo-instruction of [AND (immediate)](and_z_zi.html). This means:

- The encodings in this description are named to match the encodings of [AND (immediate)](and_z_zi.html).
- The assembler syntax is used only for assembly, and is not used on disassembly.
- The description of [AND (immediate)](and_z_zi.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

### <span id="iclass_sve"></span>SVE<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="13" class="lr">imm13</td>
<td colspan="5" class="lr">Zdn</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="13"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="bic_z_zi__and_z_zi_"></span>

BIC [\<Zdn\>](#Zdn__2 "Is the name of the source and destination scalable vector register, encoded in the "Zdn" field.").[\<T\>](#T__12 "Is the size specifier, "), [\<Zdn\>](#Zdn__2 "Is the name of the source and destination scalable vector register, encoded in the "Zdn" field.").[\<T\>](#T__12 "Is the size specifier, "), \#[\<const\>](#const "Is a 64, 32, 16 or 8-bit bitmask consisting of replicated 2, 4, 8, 16, 32 or 64 bit fields, each field containing a rotated run of non-zero bits, encoded in the "imm13" field.")

is equivalent to

[AND](and_z_zi.html#and_z_zi_) [\<Zdn\>](and_z_zi.html#Zdn__2).[\<T\>](and_z_zi.html#T__12), [\<Zdn\>](and_z_zi.html#Zdn__2).[\<T\>](and_z_zi.html#T__12), \#(-[\<const\>](and_z_zi.html#const) - 1)

</div>

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zdn&gt;</td>
<td><span id="Zdn__2"></span>
<p>Is the name of the source and destination scalable vector register, encoded in the "Zdn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T__12"></span>
<p>Is the size specifier, encoded in “imm13”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">imm13</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0xxxxxx0xxxxx</td>
<td class="symbol">S</td>
</tr>
<tr>
<td class="bitfield">0xxxxxx10xxxx</td>
<td class="symbol">H</td>
</tr>
<tr>
<td class="bitfield">0xxxxxx110xxx</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">0xxxxxx1110xx</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">0xxxxxx11110x</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">0xxxxxx11111x</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">1xxxxxxxxxxxx</td>
<td class="symbol">D</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;const&gt;</td>
<td><span id="const"></span>
<p>Is a 64, 32, 16 or 8-bit bitmask consisting of replicated 2, 4, 8, 16, 32 or 64 bit fields, each field containing a rotated run of non-zero bits, encoded in the "imm13" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [AND (immediate)](and_z_zi.html) gives the operational pseudocode for this instruction.

</div>

<div class="alias_ps_section">

### Operational Information

The description of [AND (immediate)](and_z_zi.html) gives the operational information for this instruction.

</div>
