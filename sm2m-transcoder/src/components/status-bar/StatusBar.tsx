import React from "react";

import "./StatusBar.css";

export const StatusBar: React.FC = () => {
  const driverVersion = () => window.native?.version()
  const libusbVersion = () => window.native?.libusb_version()

  return (
    <div className="status-bar">
      <div>Driver version: {driverVersion()}</div>
      <div>Libusb version: {libusbVersion()}</div>
    </div>
  );
}
