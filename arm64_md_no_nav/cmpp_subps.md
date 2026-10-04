## CMPP

Compare with tag

This instruction subtracts the 56-bit address held in the second source register from the 56-bit address held in the first source register, updates the condition flags based on the result of the subtraction, and discards the result.

This is an alias of [SUBPS](subps.html). This means:

- The encodings in this description are named to match the encodings of [SUBPS](subps.html).
- The description of [SUBPS](subps.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

### <span id="iclass_integer"></span>Integer<span style="font-size:smaller;">\
(FEAT_MTE) </span>

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
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rm</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
</tr>
<tr class="secondrow">
<td class="droppedname">sf</td>
<td></td>
<td class="droppedname">S</td>
<td></td>
<td colspan="3"></td>
<td colspan="4"></td>
<td colspan="5"></td>
<td colspan="6" class="droppedname">opcode</td>
<td colspan="5"></td>
<td colspan="5" class="droppedname">Rd</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="CMPP_SUBPS_64S_dp_2src"></span>

CMPP [\<Xn\|SP\>](#XnSP_option__6 "Is the 64-bit name of the first source general-purpose register or stack pointer, encoded in the "Rn" field."), [\<Xm\|SP\>](#XmSP_option__2 "Is the 64-bit name of the second general-purpose source register or stack pointer, encoded in the "Rm" field.")

is equivalent to

[SUBPS](subps.html#SUBPS_64S_dp_2src) XZR, [\<Xn\|SP\>](subps.html#XnSP_option__6), [\<Xm\|SP\>](subps.html#XmSP_option__2)

and is always the preferred disassembly.

</div>

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Xn|SP&gt;</td>
<td><span id="XnSP_option__6"></span>
<p>Is the 64-bit name of the first source general-purpose register or stack pointer, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xm|SP&gt;</td>
<td><span id="XmSP_option__2"></span>
<p>Is the 64-bit name of the second general-purpose source register or stack pointer, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [SUBPS](subps.html) gives the operational pseudocode for this instruction.

</div>
