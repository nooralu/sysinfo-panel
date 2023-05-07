import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import defaultConfig from "./config";

const defaultData = {
  upload: "0.00 KB/s",
  download: "0.00 KB/s",
  cpu: "0%",
  memory: "0%",
};

let cachedConfig = {};
let lastLeftClickTime = 0;
let lastRightClickTime = 0;

function App() {
  const [data, setData] = useState(defaultData);
  const [currLocale, setcurrLocale] = useState(defaultConfig.locales[0]);
  const [currTheme, setCurrTheme] = useState(defaultConfig.themes[0]);

  useEffect(() => {
    listen("system-info", (event) => {
      setData(JSON.parse(event.payload));
    });
    invoke("get_config")
      .then((c) => {
        cachedConfig = JSON.parse(c);
        console.log(cachedConfig);
        let locale = cachedConfig.locales.find(
          (item) => item.name === currLocale.name
        );
        if (!locale) {
          locale = defaultConfig.locales[0];
        }
        setcurrLocale(locale);

        let theme = cachedConfig.themes.find(
          (item) => item.name === currTheme.name
        );
        if (!theme) {
          theme = defaultConfig.themes[0];
        }
        setCurrTheme(theme);
      })
      .catch((error) => {
        console.error(error);
      });
  }, []);

  async function hanleKeyDown(e) {
    // left click, next theme
    if (e.button === 0) {
      if (Date.now() - lastLeftClickTime < 500) {
        const index = cachedConfig.themes.findIndex(
          (item) => item.name === currTheme.name
        );
        const nextIndex = (index + 1) % cachedConfig.themes.length;
        const nextTheme = cachedConfig.themes[nextIndex];
        setCurrTheme(nextTheme);
      }
      lastLeftClickTime = Date.now();
    }
    // right click, next locale
    if (e.button === 2) {
        if (Date.now() - lastRightClickTime < 500) {
        const index = cachedConfig.locales.findIndex(
          (item) => item.name === currLocale.name
        );
        const nextIndex = (index + 1) % cachedConfig.locales.length;
        const nextLocale = cachedConfig.locales[nextIndex];
        setcurrLocale(nextLocale);
      }
      lastRightClickTime = Date.now();
    }
  }


  return (
    <div className="container">
      <div className="info" style={currTheme}>
        <div className="row">
          <Cell label={currLocale.upload} value={data.upload} />
          <Cell label={currLocale.download} value={data.download} />
        </div>
        <div className="row">
          <Cell label={currLocale.cpu} value={data.cpu} />
          <Cell label={currLocale.memory} value={data.memory} />
        </div>
      </div>
      <div
        className="dragg-region"
        data-tauri-drag-region
        onMouseDown={hanleKeyDown}
      ></div>
    </div>
  );
}

function Cell({ label, value }) {
  return (
    <div className="cell">
      <span className="label">{label}</span>
      <span className="value">{value}</span>
    </div>
  );
}

export default App;
