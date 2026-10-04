## FMOV (zero, predicated)

Move floating-point +0.0 to vector elements (predicated)

This instruction moves floating-point constant +0.0 to each Active element in the destination vector. Inactive elements in the destination vector register remain unmodified.

This is a pseudo-instruction of [CPY (immediate, merging)](cpy_z_p_i.html). This means:

- The encodings in this description are named to match the encodings of [CPY (immediate, merging)](cpy_z_p_i.html).
- The assembler syntax is used only for assembly, and is not used on disassembly.
- The description of [CPY (immediate, merging)](cpy_z_p_i.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

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
<td colspan="2" class="lr">size</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="4" class="lr">Pg</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td class="droppedname">M</td>
<td class="droppedname">sh</td>
<td colspan="8" class="droppedname">imm8</td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fmov_z_p_0__cpy_z_p_i_"></span>

FMOV [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").[\<T\>](#T__104 "Is the size specifier, "), [\<Pg\>](#Pg__2 "Is the name of the governing scalable predicate register, encoded in the "Pg" field.")/M, \#0.0

is equivalent to

[CPY](cpy_z_p_i.html#cpy_z_p_i_) [\<Zd\>](cpy_z_p_i.html#Zd).[\<T\>](cpy_z_p_i.html#T__104), [\<Pg\>](cpy_z_p_i.html#Pg__2)/M, \#0

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
<td><span id="T__104"></span>
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
<td class="symbol">RESERVED</td>
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
<td>&lt;Pg&gt;</td>
<td><span id="Pg__2"></span>
<p>Is the name of the governing scalable predicate register, encoded in the "Pg" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [CPY (immediate, merging)](cpy_z_p_i.html) gives the operational pseudocode for this instruction.

</div>

<div class="alias_ps_section">

### Operational Information

The description of [CPY (immediate, merging)](cpy_z_p_i.html) gives the operational information for this instruction.

</div>
