## ADR

Form PC-relative address

This instruction adds an immediate value to the PC value to form a PC-relative address, and writes the result to the destination register.

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
<td class="lr">0</td>
<td colspan="2" class="lr">immlo</td>
<td class="l">1</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="19" class="lr">immhi</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td class="droppedname">op</td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="19"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="ADR_only_pcreladdr"></span>

ADR [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<label\>](#immhiimmlo_offset "Is the program label whose address is to be calculated. Its offset from the address of this instruction, in the range +/-1MB, is encoded in "immhi:immlo".")

</div>

#### Decode for this encoding

let d : integer{} = UInt(Rd); let imm : bits(64) = SignExtend{}(immhi::immlo);

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
<td>&lt;label&gt;</td>
<td><span id="immhiimmlo_offset"></span>
<p>Is the program label whose address is to be calculated. Its offset from the address of this instruction, in the range +/-1MB, is encoded in "immhi:immlo".</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[X](shared_pseudocode.html#accessor_X_2){64}(d) = [PC64](shared_pseudocode.html#func_PC64_0)() + imm;

</div>
