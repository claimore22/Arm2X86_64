## FACLE

Floating-point absolute compare less than or equal

This instruction compares active absolute values of floating-point elements in the first source vector being less than or equal to the corresponding absolute values of elements in the second source vector, and places the boolean results of the comparison in the corresponding elements of the destination predicate. Inactive elements in the destination predicate register are set to zero. This instruction does not set the condition flags.

This is a pseudo-instruction of [FAC\<cc\>](facge_p_p_zz.html). This means:

- The encodings in this description are named to match the encodings of [FAC\<cc\>](facge_p_p_zz.html).
- The assembler syntax is used only for assembly, and is not used on disassembly.
- The description of [FAC\<cc\>](facge_p_p_zz.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

### <span id="iclass_greater_than_or_equal"></span>Greater than or equal<span style="font-size:smaller;">\
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
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">size</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Zm</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">1</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="5"></td>
<td class="droppedname">op</td>
<td></td>
<td class="droppedname">o2</td>
<td colspan="3"></td>
<td colspan="5"></td>
<td class="droppedname">o3</td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="facle_p_p_zz__facge_p_p_zz_"></span>

FACLE [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").[\<T\>](#T__39 "Is the size specifier, "), [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/Z, [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").[\<T\>](#T__39 "Is the size specifier, "), [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__39 "Is the size specifier, ")

is equivalent to

[FACGE](facge_p_p_zz.html#facge_p_p_zz_) [\<Pd\>](facge_p_p_zz.html#Pd).[\<T\>](facge_p_p_zz.html#T__39), [\<Pg\>](facge_p_p_zz.html#Pg)/Z, [\<Zn\>](facge_p_p_zz.html#Zn__2).[\<T\>](facge_p_p_zz.html#T__39), [\<Zm\>](facge_p_p_zz.html#Zm).[\<T\>](facge_p_p_zz.html#T__39)

</div>

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Pd&gt;</td>
<td><span id="Pd"></span>
<p>Is the name of the destination scalable predicate register, encoded in the "Pd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T__39"></span>
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
<td><span id="Pg"></span>
<p>Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zm&gt;</td>
<td><span id="Zm"></span>
<p>Is the name of the second source scalable vector register, encoded in the "Zm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn&gt;</td>
<td><span id="Zn__2"></span>
<p>Is the name of the first source scalable vector register, encoded in the "Zn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [FAC\<cc\>](facge_p_p_zz.html) gives the operational pseudocode for this instruction.

</div>

<div class="alias_ps_section">

### Operational Information

The description of [FAC\<cc\>](facge_p_p_zz.html) gives the operational information for this instruction.

</div>
