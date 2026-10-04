## MOVZ

Move wide with zero

This instruction moves an optionally-shifted 16-bit immediate value to a register.

This instruction is used by the alias [MOV (wide immediate)](mov_movz.html "Move wide immediate value").

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

<span id="MOVZ_32_movewide"></span> Applies when <span class="bitdiff"> (sf == 0 && hw == 0x)</span>

MOVZ [\<Wd\>](#WdOrWZR "Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field."), \#[\<imm\>](#imm__18 "Is the 16-bit unsigned immediate, in the range 0 to 65535, encoded in the "imm16" field."){, LSL \#[\<shift\>](#shift__7 "For the "32-bit" variant: is the amount by which to shift the immediate left, either 0 (the default) or 16, encoded in the "hw" field as <shift>/16.")}

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="MOVZ_64_movewide"></span> Applies when <span class="bitdiff"> (sf == 1)</span>

MOVZ [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), \#[\<imm\>](#imm__18 "Is the 16-bit unsigned immediate, in the range 0 to 65535, encoded in the "imm16" field."){, LSL \#[\<shift\>](#shift__8 "For the "64-bit" variant: is the amount by which to shift the immediate left, either 0 (the default), 16, 32 or 48, encoded in the "hw" field as <shift>/16.")}

</div>

#### Decode for all variants of this encoding

if sf == '0' && hw\[1\] == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = UInt(Rd); let datasize : integer{} = 32 \<\< UInt(sf); let imm : bits(16) = imm16; let pos : integer{} = UInt(hw) \<\< 4;

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
<td><span id="imm__18"></span>
<p>Is the 16-bit unsigned immediate, in the range 0 to 65535, encoded in the "imm16" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;shift&gt;</td>
<td><span id="shift__7"></span>
<p>For the "32-bit" variant: is the amount by which to shift the immediate left, either 0 (the default) or 16, encoded in the "hw" field as &lt;shift&gt;/16.</p></td>
</tr>
<tr>
<td></td>
<td><span id="shift__8"></span>
<p>For the "64-bit" variant: is the amount by which to shift the immediate left, either 0 (the default), 16, 32 or 48, encoded in the "hw" field as &lt;shift&gt;/16.</p></td>
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

### Alias Conditions

| Alias | Is preferred when |
|----|----|
| [MOV (wide immediate)](mov_movz.html "Move wide immediate value") | <span class="pseudocode">!(IsZero(imm16) && hw != '00')</span> |

<div class="ps">

### Operation

var result : bits(datasize) = Zeros{}; result\[pos+15:pos\] = imm; [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result;

</div>
