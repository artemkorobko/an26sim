# SM2M Transcoder
SM2M Transcoder is the UI application which can read and write aircraft parameters to SM2M encoder and decoder.

![Transcoder](../doc/transcoder.png "Transcoder")

## Install dependencies
To install all required dependencies execute the following command:
```bash
npm ci
```

# Run application in development mode
Application uses the native NodeJS module to communicate with decoder and encoder hardware via USB driver. We need to build it first using the following command:
```bash
npm run native:build
```

Then we can start application itself:
```bash
npm run electron:dev
```

# Build release version

## Build native module
We need to build release version of the native module using the following command:
```bash
npm run native:build-release
```

Then we can build the application to `dist` directory:
```bash
npm run electron:build
```
