import React from "react";
import { Version } from "../../domain";

import "./StatusBar.css";

const DEFAULT_VERSION = "unknown";

interface StatusBarProps {
  version?: Version,
}

export const StatusBar: React.FC<StatusBarProps> = (props: StatusBarProps) => {
  return (
    <div className="status-bar">
      <div>Driver version: {props.version?.driver || DEFAULT_VERSION}</div>
      <div>Libusb version: {props.version?.libusb || DEFAULT_VERSION}</div>
    </div>
  );
}
