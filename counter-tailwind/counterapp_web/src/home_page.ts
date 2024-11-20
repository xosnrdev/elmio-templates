import { Elmio, defaultDebugConfig } from "elmio-js";

import init, { homePage } from "../wasm/counterapp.js";
(async () => {
    await init("/wasm/counterapp_bg.wasm");

    const elmio = new Elmio(homePage(location.href), {
        loggerConfig: defaultDebugConfig(),
    });

    elmio.init();
})();
