## DMB

Data memory barrier

This instruction is a memory barrier that ensures the ordering of observations of memory accesses, see <span class="armarm-xref" data-linkend="ARMARM_BEIIECBH">Data Memory Barrier</span>.

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
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td class="r">1</td>
<td colspan="4" class="lr">CRm</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="14"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td colspan="5" class="droppedname">Rt</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="DMB_BO_barriers"></span>

DMB ([\<option\>](#CRm_option__3 "Specifies the limitation on the Memory Effects ordered by the barrier operation, and is encoded in "CRm".  For more information on whether a Memory Effect is before or after a barrier instruction, see x[Data Memory Barrier (DMB)](BEIIECBH).")\|#[\<imm\>](#option__2 "Is a 4-bit unsigned immediate, in the range 0 to 15, encoded in the "CRm" field."))

</div>

#### Decode for this encoding

var types : [MBReqTypes](shared_pseudocode.html#type_MBReqTypes); case CRm\[1:0\] of when '00' =\> types = [MBReqTypes_All](shared_pseudocode.html#enum_MBReqTypes_All); when '01' =\> types = [MBReqTypes_Reads](shared_pseudocode.html#enum_MBReqTypes_Reads); when '10' =\> types = [MBReqTypes_Writes](shared_pseudocode.html#enum_MBReqTypes_Writes); when '11' =\> types = [MBReqTypes_All](shared_pseudocode.html#enum_MBReqTypes_All); end;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;option&gt;</td>
<td><span id="CRm_option__3"></span>
<p>Specifies the limitation on the Memory Effects ordered by the barrier operation, and is encoded in "CRm".</p>
<p>For more information on whether a Memory Effect is before or after a barrier instruction, see <span class="armarm-xref" data-linkend="BEIIECBH">Data Memory Barrier (DMB)</span>.</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">CRm</th>
<th class="symbol">&lt;option&gt;</th>
<th class="symbol">Description</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">xx00</td>
<td class="symbol">RESERVED</td>
<td class="description"></td>
</tr>
<tr>
<td class="bitfield">0001</td>
<td class="symbol">OSHLD</td>
<td class="description"><p>This option has the same behavior as LD and is deprecated.</p></td>
</tr>
<tr>
<td class="bitfield">0010</td>
<td class="symbol">OSHST</td>
<td class="description"><p>This option has the same behavior as ST and is deprecated.</p></td>
</tr>
<tr>
<td class="bitfield">0011</td>
<td class="symbol">OSH</td>
<td class="description"><p>This option has the same behavior as SY and is deprecated.</p></td>
</tr>
<tr>
<td class="bitfield">0101</td>
<td class="symbol">NSHLD</td>
<td class="description"><p>This option has the same behavior as LD and is deprecated.</p></td>
</tr>
<tr>
<td class="bitfield">0110</td>
<td class="symbol">NSHST</td>
<td class="description"><p>This option has the same behavior as ST and is deprecated.</p></td>
</tr>
<tr>
<td class="bitfield">0111</td>
<td class="symbol">NSH</td>
<td class="description"><p>This option has the same behavior as SY and is deprecated.</p></td>
</tr>
<tr>
<td class="bitfield">1001</td>
<td class="symbol">ISHLD</td>
<td class="description"><p>This option has the same behavior as LD and is deprecated.</p></td>
</tr>
<tr>
<td class="bitfield">1010</td>
<td class="symbol">ISHST</td>
<td class="description"><p>This option has the same behavior as ST and is deprecated.</p></td>
</tr>
<tr>
<td class="bitfield">1011</td>
<td class="symbol">ISH</td>
<td class="description"><p>This option has the same behavior as SY and is deprecated.</p></td>
</tr>
<tr>
<td class="bitfield">1101</td>
<td class="symbol">LD</td>
<td class="description"><p>Reads are the required access type before the barrier instruction. Reads and writes are the required access types after the barrier instruction.</p></td>
</tr>
<tr>
<td class="bitfield">1110</td>
<td class="symbol">ST</td>
<td class="description"><p>Writes are the required access type, both before and after the barrier instruction.</p></td>
</tr>
<tr>
<td class="bitfield">1111</td>
<td class="symbol">SY</td>
<td class="description"><p>Reads and writes are the required access types, both before and after the barrier instruction.</p></td>
</tr>
</tbody>
</table>
All encodings of "CRm" that are listed as reserved can be encoded using the #&lt;imm&gt; syntax. All reserved options have the same behavior as SY, but software must not rely on this behavior.</td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="option__2"></span>
<p>Is a 4-bit unsigned immediate, in the range 0 to 15, encoded in the "CRm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

DataMemoryBarrier(types);

</div>
