## AXFLAG

Convert floating-point condition flags from Arm to external format

This instruction converts the state of the PSTATE.{N,Z,C,V} flags from a form representing the result of an Arm floating-point scalar compare instruction to an alternative representation required by some software.

### <span id="iclass_system"></span>System<span style="font-size:smaller;">\
(FEAT_FlagM2) </span>

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
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td>1</td>
<td>0</td>
<td class="r">0</td>
<td class="l">(0)</td>
<td>(0)</td>
<td>(0)</td>
<td class="r">(0)</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="7"></td>
<td colspan="3" class="droppedname">op1</td>
<td colspan="4"></td>
<td colspan="4" class="droppedname">CRm</td>
<td colspan="3" class="droppedname">op2</td>
<td colspan="5" class="droppedname">Rt</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="AXFLAG_M_pstate"></span>

AXFLAG

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_FlagM2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end;

<div class="encoding-notes">

</div>

<div class="ps">

### Operation

let z : bit = [PSTATE](shared_pseudocode.html#global_PSTATE).Z OR [PSTATE](shared_pseudocode.html#global_PSTATE).V; let c : bit = [PSTATE](shared_pseudocode.html#global_PSTATE).C AND NOT([PSTATE](shared_pseudocode.html#global_PSTATE).V); PSTATE.\[N,Z,C,V\] = '0' :: z :: c :: '0';

</div>
