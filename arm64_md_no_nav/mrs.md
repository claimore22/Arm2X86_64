## MRS

Move System register to general-purpose register

This instruction allows the PE to read an AArch64 System register into a general-purpose register.

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
<td class="lr">1</td>
<td class="lr">o0</td>
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
<td></td>
<td></td>
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

<span id="MRS_RS_systemmove"></span>

MRS [\<Xt\>](#XtOrXZR__4 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rt" field."), ([\<systemreg\>](#MRS_values "Is a System register name, encoded in "o0:op1:CRn:CRm:op2".  The System register names are defined in x[AArch64 System Register Descriptions](autogen.registers.AArch64).")\|S[\<op0\>](#op0_option__2 "Is an unsigned immediate, ")\_[\<op1\>](#op1 "Is a 3-bit unsigned immediate, in the range 0 to 7, encoded in the "op1" field.")\_[\<Cn\>](#Cn "Is a name 'Cn', with 'n' in the range 0 to 15, encoded in the "CRn" field.")\_[\<Cm\>](#Cm "Is a name 'Cm', with 'm' in the range 0 to 15, encoded in the "CRm" field.")\_[\<op2\>](#op2 "Is a 3-bit unsigned immediate, in the range 0 to 7, encoded in the "op2" field."))

</div>

#### Decode for this encoding

let t : integer = UInt(Rt); let sys_L : bits(1) = L; let sys_op0 : bits(2) = '1' :: o0; let sys_op1 : bits(3) = op1; let sys_op2 : bits(3) = op2; let sys_crn : bits(4) = CRn; let sys_crm : bits(4) = CRm;

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
<td>&lt;systemreg&gt;</td>
<td><span id="MRS_values"></span>
<p>Is a System register name, encoded in "o0:op1:CRn:CRm:op2".</p>
<p>The System register names are defined in <span class="armarm-xref" data-linkend="autogen.registers.AArch64">AArch64 System Register Descriptions</span>.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;op0&gt;</td>
<td><span id="op0_option__2"></span>
<p>Is an unsigned immediate, encoded in “o0”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">o0</th>
<th class="symbol">&lt;op0&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">2</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">3</td>
</tr>
</tbody>
</table></td>
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

<div class="ps">

### Operation

AArch64_SysRegRead(sys_op0, sys_op1, sys_crn, sys_crm, sys_op2, t);

</div>
