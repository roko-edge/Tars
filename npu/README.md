# Ambiente em SystemVerilog

Espaço para você escrever e testar seus módulos livremente em **SystemVerilog**.

---

## Comandos

Dentro desta pasta (`npu/`), use:

```bash
make sim     # Compila com iverilog (-g2012) e executa no terminal
make wave    # Abre as ondas geradas no GTKWave
make lint    # Análise estática com Verilator
make clean   # Limpa a pasta build/
```

Para carregar o ambiente com as ferramentas:
```bash
nix develop .#hardware
```
