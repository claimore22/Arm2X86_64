## MOV (vector, unpredicated)

Move vector register (unpredicated)

This instruction copies the contents of the source vector register to the destination vector register. This instruction is unpredicated.

This is an alias of [ORR (vectors, unpredicated)](orr_z_zz.html). This means:

- The encodings in this description are named to match the encodings of [ORR (vectors, unpredicated)](orr_z_zz.html).
- The description of [ORR (vectors, unpredicated)](orr_z_zz.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

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
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zm</td>
<td class="l">0</td>
<td>0</td>
<td class="r">1</td>
<td class="l">1</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="5"></td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="mov_z_z__orr_z_zz_"></span>

MOV [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").D, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").D

is equivalent to

[ORR](orr_z_zz.html#orr_z_zz_) [\<Zd\>](orr_z_zz.html#Zd).D, [\<Zn\>](orr_z_zz.html#Zn__2).D, [\<Zn\>](orr_z_zz.html#Zn__2).D

and is the preferred disassembly when <span class="pseudocode">Zn == Zm</span>.

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
<td>&lt;Zn&gt;</td>
<td><span id="Zn"></span>
<p>Is the name of the source scalable vector register, encoded in the "Zn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [ORR (vectors, unpredicated)](orr_z_zz.html) gives the operational pseudocode for this instruction.

</div>

<div class="alias_ps_section">

### Operational Information

The description of [ORR (vectors, unpredicated)](orr_z_zz.html) gives the operational information for this instruction.

</div>
