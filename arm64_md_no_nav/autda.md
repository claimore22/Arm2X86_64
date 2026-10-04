## AUTDA, AUTDZA

Authenticate data address, using key A

This instruction authenticates a data address, using a modifier and key A.

The address is in the general-purpose register that is specified by \<Xd\>.

The modifier is:

- In the general-purpose register or stack pointer that is specified by \<Xn\|SP\>, for <span class="asm-code">AUTDA</span>.
- The value zero, for <span class="asm-code">AUTDZA</span>.

If the authentication passes, the upper bits of the address are restored to enable subsequent use of the address. For information on behavior if the authentication fails, see <span class="armarm-xref" data-linkend="ARMARM_MDSec.Faulting_on_pointer_authentication">Faulting on pointer authentication</span>.

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
<td class="lr">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">Z</td>
<td class="l">1</td>
<td>1</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td class="droppedname">sf</td>
<td></td>
<td class="droppedname">S</td>
<td colspan="8"></td>
<td colspan="5" class="droppedname">opcode2</td>
<td colspan="2"></td>
<td></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the AUTDA variant

<span id="AUTDA_64P_dp_1src"></span> Applies when <span class="bitdiff"> (Z == 0)</span>

AUTDA [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Xn\|SP\>](#XnSP_option__7 "Is the 64-bit name of the general-purpose source register or stack pointer, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the AUTDZA variant

<span id="AUTDZA_64Z_dp_1src"></span> Applies when <span class="bitdiff"> (Z == 1 && Rn == 11111)</span>

AUTDZA [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field.")

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_PAuth) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if Z == '1' && Rn != '11111' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let auth_combined : boolean = FALSE; let d : integer = UInt(Rd); let n : integer = UInt(Rn); let source_is_sp : boolean = Z == '0' && n == 31;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Xd&gt;</td>
<td><span id="XdOrXZR__6"></span>
<p>Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xn|SP&gt;</td>
<td><span id="XnSP_option__7"></span>
<p>Is the 64-bit name of the general-purpose source register or stack pointer, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

if source_is_sp then [X](shared_pseudocode.html#accessor_X_2){64}(d) = [AuthDA](shared_pseudocode.html#func_AuthDA_3)([X](shared_pseudocode.html#accessor_X_2){64}(d), [SP](shared_pseudocode.html#accessor_SP_1){64}(), auth_combined); else [X](shared_pseudocode.html#accessor_X_2){64}(d) = [AuthDA](shared_pseudocode.html#func_AuthDA_3)([X](shared_pseudocode.html#accessor_X_2){64}(d), [X](shared_pseudocode.html#accessor_X_2){64}(n), auth_combined); end;

</div>
