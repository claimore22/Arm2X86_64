## SYSL

System instruction with result

For more information, see <span class="armarm-xref" data-linkend="ARMARM_BABEJJJE">op0 == 0b01, cache maintenance, TLB maintenance, address translation, prediction restriction, BRBE, Trace Extension, and Guarded Control Stack instructions</span> for the encodings of System instructions.

This instruction is used by the aliases [GCSPOPM](gcspopm_sysl.html "Guarded Control Stack pop"), and [GCSSS2](gcsss2_sysl.html "Guarded Control Stack switch stack 2").

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
<td class="l">1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="3" class="lr">op1</td>
<td colspan="4" class="lr">CRn</td>
<td colspan="4" class="lr">CRm</td>
<td colspan="3" class="lr">op2</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="4"></td>
<td class="droppedname">L</td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="4"></td>
<td colspan="4"></td>
<td colspan="3"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="SYSL_RC_systeminstrs"></span>

SYSL [\<Xt\>](#XtOrXZR__4 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rt" field."), \#[\<op1\>](#op1 "Is a 3-bit unsigned immediate, in the range 0 to 7, encoded in the "op1" field."), [\<Cn\>](#Cn "Is a name 'Cn', with 'n' in the range 0 to 15, encoded in the "CRn" field."), [\<Cm\>](#Cm "Is a name 'Cm', with 'm' in the range 0 to 15, encoded in the "CRm" field."), \#[\<op2\>](#op2 "Is a 3-bit unsigned immediate, in the range 0 to 7, encoded in the "op2" field.")

</div>

#### Decode for this encoding

let t : integer = UInt(Rt); let sys_L : bits(1) = L; let sys_op0 : bits(2) = '01'; let sys_op1 : bits(3) = op1; let sys_op2 : bits(3) = op2; let sys_crn : bits(4) = CRn; let sys_crm : bits(4) = CRm;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Xt&gt;</td>
<td><span id="XtOrXZR__4"></span>
<p>Is the 64-bit name of the general-purpose destination register, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;op1&gt;</td>
<td><span id="op1"></span>
<p>Is a 3-bit unsigned immediate, in the range 0 to 7, encoded in the "op1" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Cn&gt;</td>
<td><span id="Cn"></span>
<p>Is a name 'Cn', with 'n' in the range 0 to 15, encoded in the "CRn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Cm&gt;</td>
<td><span id="Cm"></span>
<p>Is a name 'Cm', with 'm' in the range 0 to 15, encoded in the "CRm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;op2&gt;</td>
<td><span id="op2"></span>
<p>Is a 3-bit unsigned immediate, in the range 0 to 7, encoded in the "op2" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

### Alias Conditions

| Alias | Is preferred when |
|----|----|
| [GCSPOPM](gcspopm_sysl.html "Guarded Control Stack pop") | <span class="pseudocode">op1 == '011' && CRn == '0111' && CRm == '0111' && op2 == '001'</span> |
| [GCSSS2](gcsss2_sysl.html "Guarded Control Stack switch stack 2") | <span class="pseudocode">op1 == '011' && CRn == '0111' && CRm == '0111' && op2 == '011'</span> |

<div class="ps">

### Operation

AArch64_SysInstrWithResult(sys_op0, sys_op1, sys_crn, sys_crm, sys_op2, t);

</div>
