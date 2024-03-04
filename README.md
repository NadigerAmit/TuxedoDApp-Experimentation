# TuxedoDApp-Experimentation
This project serves as a playground for building the TuxedoDApp by reusing Rust code, particularly focusing on the wallet functionality. The primary aim is to compile Rust code to WebAssembly (Wasm) using wasm-bindgen.
**Introduction**
This repository serves as an experimentation ground for building the TuxedoDApp, leveraging existing Rust code for the wallet functionality from Off-Narrative-Labs/Tuxedo: https://github.com/Off-Narrative-Labs/Tuxedo/tree/main/wallet. The goal is to compile the Rust code into WebAssembly using wasm-bindgen, enabling smooth interaction with web applications.

**Getting Started**

**1. Clone the Repository:**

```https://github.com/NadigerAmit/TuxedoDApp-Experimentation.git```

**2. Compile the Code:**

```wasm-pack build --target web```

**3. Run the DApp:**
Serve the root directory of the project with a local web server. For instance, you can use Python's built-in server:
```python3 -m http.server```

Load index.html from the web server by navigating to http://localhost:8000 in your web browser.

An alert box will appear on the screen, displaying the message "Hello Greeting from Tuxedo DApp, WebAssembly!"

To further explore functionality, check the browser console. You should observe the message "Gett Kitty info called from Rust code." This output is generated by invoking the Rust code through the get_kitty_info() API.

