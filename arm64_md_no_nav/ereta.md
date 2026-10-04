## ERETAA, ERETAB

Exception return, with pointer authentication

This instruction authenticates the address in ELR, using SP as the modifier and the specified key, restores <span class="armarm-xref" data-linkend="ARMARM_PSTATE">PSTATE</span> from the SPSR for the current Exception level, and branches to the authenticated address.

Key A is used for <span class="asm-code">ERETAA</span>. Key B is used for <span class="asm-code">ERETAB</span>.

If the authentication passes, the PE continues execution at the target of the branch. For information on behavior if the authentication fails, see <span class="armarm-xref" data-linkend="ARMARM_MDSec.Faulting_on_pointer_authentication">Faulting on pointer authentication</span>.

The authenticated address is not written back to ELR.

The SPSR is checked for the current Exception level for an illegal return event. See <span class="armarm-xref" data-linkend="ARMARM_MDSec.illegal_exception_returns_from_aarch64_state">Illegal exception returns from AArch64 state</span>.

<span class="asm-code">ERETAA</span> and <span class="asm-code">ERETAB</span> are <span class="arm-defined-word">UNDEFINED</span> at EL0.

### <span id="iclass_integer"></span>Integer<span style="font-size:smaller;">\
(FEAT_PAuth) </span>

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
<td>0</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td class="r">1</td>
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
<td class="lr">1</td>
<td class="lr">M</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
</tr>
<tr class="secondrow">
<td colspan="7"></td>
<td colspan="4" class="droppedname">opc</td>
<td colspan="5" class="droppedname">op2</td>
<td colspan="4"></td>
<td class="droppedname">A</td>
<td></td>
<td colspan="5" class="droppedname">Rn</td>
<td colspan="5" class="droppedname">op4</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the ERETAA variant

<span id="ERETAA_64E_branch_reg"></span> Applies when <span class="bitdiff"> (M == 0)</span>

ERETAA

</div>

<div class="encoding">

#### Encoding for the ERETAB variant

<span id="ERETAB_64E_branch_reg"></span> Applies when <span class="bitdiff"> (M == 1)</span>

ERETAB

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_PAuth) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let pac : boolean = TRUE; let use_key_a : boolean = (M == '0'); let auth_then_branch : boolean = TRUE;

<div class="encoding-notes">

</div>

<div class="ps">

### Operation

if [PSTATE](shared_pseudocode.html#global_PSTATE).EL == [EL0](shared_pseudocode.html#global_EL0) then Undefined(); end; [AArch64_CheckForERetTrap](shared_pseudocode.html#func_AArch64_CheckForERetTrap_2)(pac, use_key_a); var target : bits(64) = [ELR_ELx](shared_pseudocode.html#accessor_ELR_ELx_0)(); let modifier : bits(64) = SP{}(); if use_key_a then target = [AuthIA](shared_pseudocode.html#func_AuthIA_3)(target, modifier, auth_then_branch); else target = [AuthIB](shared_pseudocode.html#func_AuthIB_3)(target, modifier, auth_then_branch); end; [AArch64_ExceptionReturn](shared_pseudocode.html#func_AArch64_ExceptionReturn_2)(target, [SPSR_ELx](shared_pseudocode.html#accessor_SPSR_ELx_0)());

</div>
