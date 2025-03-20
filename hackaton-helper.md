# 📜 Machete para el Hackathon: Rust + Soroban Smart Contracts

## 🚀 Previo al Hackathon

### 🔧 Instalación de Herramientas

#### 🖥️ Instalar Rust
- **Windows**: [Descargar Rust](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)
- **macOS/Linux**:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

#### 🎯 Instalar el target
```bash
rustup target add wasm32-unknown-unknown
```

#### 🌟 Instalar el Stellar CLI
```bash
cargo install --locked stellar-cli --feature opt
```

#### 🛠️ Instalar extensiones en VS Code
- **Rust Analyzer** → Linter para Rust
- **CodeLLDB** → Para debuggear

---

## 🏆 Durante el Hackathon

Si todo lo anterior está instalado, pasamos a crear el smart contract dentro de la carpeta del hackatón.

### 📌 Crear un Smart Contract con Soroban
```bash
stellar contract init <nombre_del_contrato>
```

---

## ✅ Deploy en Testnet

### 🔑 Generar Keypair para las pruebas
```bash
stellar keys generate --global alice --network testnet --fund
```

### 📌 Pasos para el Deploy:

1️⃣ **Compilar el contrato y generar el archivo `.wasm`**
```bash
stellar contract build
```

2️⃣ **Deployar el contrato en la Testnet y obtener el Contract ID**
```bash
stellar contract deploy \
    --wasm target/wasm32-unknown-unknown/release/hello_world.wasm \
    --source alice \
    --network testnet \
    --alias hello_world
```

---

## 📚 Documentación y Recursos

### 📖 Documentación Oficial de Stellar
🔹 [Documentación de Stellar](https://developers.stellar.org/)
🔹 [SDK de Contratos](https://developers.stellar.org/docs/tools/sdks/contract-sdks)
🔹 [SDK para Frontend/Backend](https://developers.stellar.org/docs/tools/sdks/client-sdks)
🔹 [Laboratorio de Stellar (Interfaz gráfica para interactuar con Stellar)](https://lab.stellar.org/)
🔹 [Stellar Expert (Explorador de la red)](https://stellar.expert/)
🔹 [Contratos de Ejemplo](https://developers.stellar.org/docs/build/smart-contracts/example-contracts)

### 🎙️ Comunidad y Soporte
🔹 [Discord Oficial de Stellar](https://discord.com/invite/stellar-global-761985725453303838)
🔹 [IA de Stellar para Preguntas](https://developers.stellar.org/docs/tools/developer-tools/ai-bot) *(Usar prompt en inglés comenzando con "Stella, ...")*

### ✍️ Blogs y Recursos Adicionales
🔹 [Blog de BiggerTech con artículos sobre Stellar](https://blog.biggertech.co/)
🔹 [Stellar Wallet Kit](https://github.com/Creit-Tech/Stellar-Wallets-Kit)


---

🎉 ¡Éxitos en el Hackathon!
Desde [BAF Network](https://www.blockchainacceleration.org/) y [BiggerTech](https://www.biggertech.co/), les deseamos mucha suerte en este hackathon. 🚀💡
Esperamos que esta guía les sea útil y que disfruten construyendo con Rust y Soroban. ¡A crear cosas increíbles!