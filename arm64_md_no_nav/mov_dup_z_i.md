## MOV (immediate, unpredicated)

Move signed immediate to vector elements (unpredicated)

This instruction unconditionally broadcasts the signed integer immediate into each element of the destination vector. This instruction is unpredicated.

The immediate operand is a signed value in the range -128 to +127, and for element widths of 16 bits or higher it may also be a signed multiple of 256 in the range -32768 to +32512 (excluding 0).

The immediate is encoded in 8 bits with an optional left shift by 8. The preferred disassembly when the shift option is specified is "#\<imm\>, LSL \#8". However an assembler and disassembler may also allow use of the shifted 16-bit value unless the immediate is 0 and the shift amount is 8, which must be unambiguously described as "#0, LSL \#8".

This is an alias of [DUP (immediate)](dup_z_i.html). This means:

- The encodings in this description are named to match the encodings of [DUP (immediate)](dup_z_i.html).
- The description of [DUP (immediate)](dup_z_i.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

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
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">size</td>
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">sh</td>
<td colspan="8" class="lr">imm8</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="2"></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="8"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="mov_z_i__dup_z_i_"></span>

MOV [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").[\<T\>](#T__28 "Is the size specifier, "), \#[\<imm\>](#imm__46 "Is a signed immediate in the range -128 to 127, encoded in the "imm8" field."){, [\<shift\>](#shift__10 "Is the optional left shift to apply to the immediate, defaulting to LSL #0 and ")}

is equivalent to

[DUP](dup_z_i.html#dup_z_i_) [\<Zd\>](dup_z_i.html#Zd).[\<T\>](dup_z_i.html#T__28), \#[\<imm\>](dup_z_i.html#imm__46){, [\<shift\>](dup_z_i.html#shift__10)}

and is always the preferred disassembly.

</div>

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zd&gt;</td>
<td><span id="Zd"></span>
<p>Is the name of the destination scalable vector register, encoded in the "Zd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T__28"></span>
<p>Is the size specifier, encoded in “size”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">size</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">H</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="symbol">S</td>
</tr>
<tr>
<td class="bitfield">11</td>
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
<td>&lt;imm&gt;</td>
<td><span id="imm__46"></span>
<p>Is a signed immediate in the range -128 to 127, encoded in the "imm8" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;shift&gt;</td>
<td><span id="shift__10"></span>
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
<td class="symbol">LSL #8</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [DUP (immediate)](dup_z_i.html) gives the operational pseudocode for this instruction.

</div>
