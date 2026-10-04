## MOV (wide immediate)

Move wide immediate value

This instruction moves a 16-bit immediate value to a register.

This is an alias of [MOVZ](movz.html). This means:

- The encodings in this description are named to match the encodings of [MOVZ](movz.html).
- The description of [MOVZ](movz.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

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
<td class="lr">sf</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td class="r">1</td>
<td colspan="2" class="lr">hw</td>
<td colspan="16" class="lr">imm16</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td colspan="6"></td>
<td colspan="2"></td>
<td colspan="16"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="MOV_MOVZ_32_movewide"></span> Applies when <span class="bitdiff"> (sf == 0 && hw == 0x)</span>

MOV [\<Wd\>](#WdOrWZR "Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field."), \#[\<imm\>](#hw_imm16__2 "For the "32-bit" variant: is a 32-bit immediate which can be encoded in "imm16:hw".")

is equivalent to

[MOVZ](movz.html#MOVZ_32_movewide) [\<Wd\>](movz.html#WdOrWZR), \#\<imm16\>, LSL \#[\<shift\>](movz.html#shift__7)

and is the preferred disassembly when <span class="pseudocode">!(IsZero(imm16) && hw != '00')</span>.

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="MOV_MOVZ_64_movewide"></span> Applies when <span class="bitdiff"> (sf == 1)</span>

MOV [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), \#[\<imm\>](#hw_imm16__4 "For the "64-bit" variant: is a 64-bit immediate which can be encoded in "imm16:hw".")

is equivalent to

[MOVZ](movz.html#MOVZ_64_movewide) [\<Xd\>](movz.html#XdOrXZR__6), \#\<imm16\>, LSL \#[\<shift\>](movz.html#shift__8)

and is the preferred disassembly when <span class="pseudocode">!(IsZero(imm16) && hw != '00')</span>.

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
<td>&lt;imm&gt;</td>
<td><span id="hw_imm16__2"></span>
<p>For the "32-bit" variant: is a 32-bit immediate which can be encoded in "imm16:hw".</p></td>
</tr>
<tr>
<td></td>
<td><span id="hw_imm16__4"></span>
<p>For the "64-bit" variant: is a 64-bit immediate which can be encoded in "imm16:hw".</p></td>
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

The description of [MOVZ](movz.html) gives the operational pseudocode for this instruction.

</div>
