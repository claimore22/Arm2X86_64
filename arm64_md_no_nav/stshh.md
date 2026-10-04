## STSHH

Store shared hint

This instruction signals to the memory system that if the next instruction in program order generates an explicit write effect, then it is to a location that one or more other threads of execution will observe, and there is a performance benefit to ensuring that the updated value from the write to that location propagates to those other observers with minimal latency.

The thread of execution on the other observers might be polling the location using load or Load-Exclusive instructions, or may have executed a <span class="asm-code">PRFM IR</span> instruction targeting the location.

### <span id="iclass_system"></span>System<span style="font-size:smaller;">\
(FEAT_PCDPHINT) </span>

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
<td class="r">0</td>
<td class="l">0</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td class="r">x</td>
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
<td colspan="4" class="droppedname">CRm</td>
<td colspan="3" class="droppedname">op2</td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="STSHH_HI_hints"></span>

STSHH [\<policy\>](#stshh_policy "<policy> is one of:  KEEP : Signals to the memory system that there may be a performance benefit to retaining the updated location in the local cache of the updating PE.  STRM : Signals to the memory system that there may be a performance benefit to ensuring the updated location is not retained in the local cache of the updating PE.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_PCDPHINT) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; let stream : boolean = op2\[0\] == '1';

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;policy&gt;</td>
<td><span id="stshh_policy"></span>
<p>&lt;policy&gt; is one of:</p>
<dl>
<dt>KEEP</dt>
<dd>
Signals to the memory system that there may be a performance benefit to retaining the updated location in the local cache of the updating PE.
</dd>
<dt>STRM</dt>
<dd>
Signals to the memory system that there may be a performance benefit to ensuring the updated location is not retained in the local cache of the updating PE.
</dd>
</dl>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">op2[0]</th>
<th class="symbol">&lt;policy&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">KEEP</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">STRM</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

Hint_StoreShared(stream);

</div>
