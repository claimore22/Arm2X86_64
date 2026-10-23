mnemonic: STFF1B
encoding: Z_Zb
operands:
  - name: Rt
    type: v
  - name: Rn
    type: w
  - name: imm
    type: imm
semantics: |
  // Store first‑fault unsigned byte
  // Variant: scalar+scalar (Rt scalar, Rn scalar, imm)
  // ... (implementation details omitted for brevity)