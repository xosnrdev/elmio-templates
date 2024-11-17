import { Elmio, defaultDebugConfig } from "elmio-js";

import init, { homePage } from "../wasm/app.js";
(async () => {
    await init("/wasm/app_bg.wasm");

    const elmio = new Elmio(homePage(location.href), {
        loggerConfig: defaultDebugConfig(),
    });

    elmio.init();
})();
