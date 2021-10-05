import { contextBridge } from "electron";

const native = require("../../native/index.node");
contextBridge.exposeInMainWorld("native", native);
