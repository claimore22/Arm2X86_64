## ERET

Exception return

This instruction restores <span class="armarm-xref" data-linkend="ARMARM_PSTATE">PSTATE</span> from the SPSR, and branches to the address held in the ELR.

The SPSR is checked for the current Exception level for an illegal return event. See <span class="armarm-xref" data-linkend="ARMARM_MDSec.illegal_exception_returns_from_aarch64_state">Illegal exception returns from AArch64 state</span>.

<span class="asm-code">ERET</span> is <span class="arm-defined-word">UNDEFINED</span> at EL0.

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
<td class="lr">1</td>
<td class="l">0</td>
<td>1</td>
<td>0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="3"></td>
<td></td>
<td colspan="4" class="droppedname">opc</td>
<td colspan="5" class="droppedname">op2</td>
<td colspan="4"></td>
<td class="droppedname">A</td>
<td class="droppedname">M</td>
<td colspan="5" class="droppedname">Rn</td>
<td colspan="5" class="droppedname">op4</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="ERET_64E_branch_reg"></span>

ERET

</div>

#### Decode for this encoding

let pac : boolean = FALSE; let use_key_a : boolean = TRUE;

<div class="encoding-notes">

</div>

<div class="ps">

### Operation

if [PSTATE](shared_pseudocode.html#global_PSTATE).EL == [EL0](shared_pseudocode.html#global_EL0) then Undefined(); end; [AArch64_CheckForERetTrap](shared_pseudocode.html#func_AArch64_CheckForERetTrap_2)(pac, use_key_a); let target : bits(64) = [ELR_ELx](shared_pseudocode.html#accessor_ELR_ELx_0)(); [AArch64_ExceptionReturn](shared_pseudocode.html#func_AArch64_ExceptionReturn_2)(target, [SPSR_ELx](shared_pseudocode.html#accessor_SPSR_ELx_0)());

</div>
