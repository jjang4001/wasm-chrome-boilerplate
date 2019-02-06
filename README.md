# wasm-chrome-boilerplate
This is a boilerplate for making chrome extensions with typescript, scss, and wasm generated by rust. This can also be a starter for making regular web apps in wasm.

# Getting Started

Make sure you have wasm-pack downloaded before running.

1. From the root directory, run
    ```
    npm install
    npm run build
    ```

2. Then open chrome and navigate to `chrome://extensions/`. Click `Load unpacked`, and select the `~/dist` directory. When you open a new tab or the popup in the top right, you should see a list of buttons that use wasm-bindgen.

3. Alternatively, you can also run the app on a local server. Simply run
    ```
    npm run start
    ```
    and then go to `localhost:8080`.

# Remarks
Note that `~/src/popup/bootstrap.js` is a js file. This is because `ts-loader` doesn't integrate very well with asynchronous imports, which are required for importing wasm modules.
