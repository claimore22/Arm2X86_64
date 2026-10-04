## MOV (to general)

Move vector element to general-purpose register

This instruction reads the unsigned integer from the source SIMD&FP register, zero-extends it to form a 32-bit or 64-bit value, and writes the result to the destination general-purpose register.

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

This is an alias of [UMOV](umov_advsimd.html). This means:

- The encodings in this description are named to match the encodings of [UMOV](umov_advsimd.html).
- The description of [UMOV](umov_advsimd.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

### <span id="iclass_advanced_simd"></span>Advanced SIMD<span style="font-size:smaller;">\
(FEAT_AdvSIMD) </span>

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
<td class="lr">0</td>
<td class="lr">Q</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">x</td>
<td>x</td>
<td>x</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td class="droppedname">op</td>
<td colspan="8"></td>
<td colspan="5" class="droppedname">imm5</td>
<td></td>
<td colspan="4" class="droppedname">imm4</td>
<td></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="MOV_UMOV_asimdins_W_w"></span> Applies when <span class="bitdiff"> (Q == 0 && imm5 == xx100)</span>

MOV [\<Wd\>](#WdOrWZR "Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Vn\>](#Vn "Is the name of the SIMD&FP source register, encoded in the "Rn" field.").S\[[\<index\>](#index__6 "For the "32-bit" variant: is the element index encoded in "imm5<4:3>".")\]

is equivalent to

[UMOV](umov_advsimd.html#UMOV_asimdins_W_w) [\<Wd\>](umov_advsimd.html#WdOrWZR), [\<Vn\>](umov_advsimd.html#Vn).S\[[\<index\>](umov_advsimd.html#index__6)\]

and is always the preferred disassembly.

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="MOV_UMOV_asimdins_X_x"></span> Applies when <span class="bitdiff"> (Q == 1 && imm5 == x1000)</span>

MOV [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Vn\>](#Vn "Is the name of the SIMD&FP source register, encoded in the "Rn" field.").D\[[\<index\>](#index__7 "For the "64-bit" variant: is the element index encoded in "imm5<4>".")\]

is equivalent to

[UMOV](umov_advsimd.html#UMOV_asimdins_X_x) [\<Xd\>](umov_advsimd.html#XdOrXZR__6), [\<Vn\>](umov_advsimd.html#Vn).D\[[\<index\>](umov_advsimd.html#index__7)\]

and is always the preferred disassembly.

</div>

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wd&gt;</td>
<td><span id="WdOrWZR"></span>
<p>Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Vn&gt;</td>
<td><span id="Vn"></span>
<p>Is the name of the SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;index&gt;</td>
<td><span id="index__6"></span>
<p>For the "32-bit" variant: is the element index encoded in "imm5&lt;4:3&gt;".</p></td>
</tr>
<tr>
<td></td>
<td><span id="index__7"></span>
<p>For the "64-bit" variant: is the element index encoded in "imm5&lt;4&gt;".</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xd&gt;</td>
<td><span id="XdOrXZR__6"></span>
<p>Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [UMOV](umov_advsimd.html) gives the operational pseudocode for this instruction.

</div>

<div class="alias_ps_section">

### Operational Information

The description of [UMOV](umov_advsimd.html) gives the operational information for this instruction.

</div>
