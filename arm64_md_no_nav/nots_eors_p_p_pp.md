## NOTS

Bitwise invert predicate, setting the condition flags

This instruction performs a bitwise invert of each active element of the source predicate, and places the results in the corresponding elements of the destination predicate. Inactive elements in the destination predicate register are set to zero. This instruction sets the First (N), None (Z), and !Last (C) condition flags based on the predicate result, and sets the V flag to zero.

This is an alias of [EORS](eors_p_p_pp.html). This means:

- The encodings in this description are named to match the encodings of [EORS](eors_p_p_pp.html).
- The description of [EORS](eors_p_p_pp.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

### <span id="iclass_setting_the_condition_flags"></span>Setting the condition flags<span style="font-size:smaller;">\
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
<td class="lr">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="4" class="lr">Pm</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="4" class="lr">Pg</td>
<td class="lr">1</td>
<td colspan="4" class="lr">Pn</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td class="droppedname">op</td>
<td class="droppedname">S</td>
<td colspan="2"></td>
<td colspan="4"></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td class="droppedname">o2</td>
<td colspan="4"></td>
<td class="droppedname">o3</td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="nots_p_p_p_z_eors_p_p_pp_z"></span>

NOTS [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").B, [\<Pg\>](#Pg__2 "Is the name of the governing scalable predicate register, encoded in the "Pg" field.")/Z, [\<Pn\>](#Pn__3 "Is the name of the source scalable predicate register, encoded in the "Pn" field.").B

is equivalent to

[EORS](eors_p_p_pp.html#eors_p_p_pp_z) [\<Pd\>](eors_p_p_pp.html#Pd).B, [\<Pg\>](eors_p_p_pp.html#Pg__2)/Z, [\<Pn\>](eors_p_p_pp.html#Pn__2).B, [\<Pg\>](eors_p_p_pp.html#Pg__2).B

and is the preferred disassembly when <span class="pseudocode">Pm == Pg</span>.

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
<td>&lt;Pg&gt;</td>
<td><span id="Pg__2"></span>
<p>Is the name of the governing scalable predicate register, encoded in the "Pg" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Pn&gt;</td>
<td><span id="Pn__3"></span>
<p>Is the name of the source scalable predicate register, encoded in the "Pn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [EORS](eors_p_p_pp.html) gives the operational pseudocode for this instruction.

</div>

<div class="alias_ps_section">

### Operational Information

The description of [EORS](eors_p_p_pp.html) gives the operational information for this instruction.

</div>
