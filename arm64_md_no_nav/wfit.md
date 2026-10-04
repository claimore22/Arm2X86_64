## WFIT

Wait for interrupt with timeout

This instruction provides a hint that the PE can enter a low-power state and remain there until either a local timeout event or a wakeup event occurs. For more information, see <span class="armarm-xref" data-linkend="ARMARM_BEIJBEJD">Wait For Interrupt</span>.

As described in <span class="armarm-xref" data-linkend="ARMARM_BEIJBEJD">Wait For Interrupt</span>, the execution of a <span class="asm-code">WFIT</span> instruction that would otherwise cause entry to a low-power state can be trapped to a higher Exception level.

### <span id="iclass_system"></span>System<span style="font-size:smaller;">\
(FEAT_WFxT) </span>

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
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td class="r">1</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="14"></td>
<td colspan="4" class="droppedname">CRm</td>
<td colspan="3" class="droppedname">op2</td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="WFIT_only_systeminstrswithreg"></span>

WFIT [\<Xt\>](#XtOrXZR__5 "Is the 64-bit name of the general-purpose source register, encoded in the "Rd" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_WFxT) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = UInt(Rd);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Xt&gt;</td>
<td><span id="XtOrXZR__5"></span>
<p>Is the 64-bit name of the general-purpose source register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

let localtimeout : integer = UInt([X](shared_pseudocode.html#accessor_X_2){64}(d)); if [Halted](shared_pseudocode.html#func_Halted_0)() && [ConstrainUnpredictableBool](shared_pseudocode.html#func_ConstrainUnpredictableBool_1)([Unpredictable_WFxTDEBUG](shared_pseudocode.html#enum_Unpredictable_WFxTDEBUG)) then [ExecuteAsNOP](shared_pseudocode.html#func_ExecuteAsNOP_0)(); end; [Hint_WFIT](shared_pseudocode.html#func_Hint_WFIT_1)(localtimeout);

</div>
