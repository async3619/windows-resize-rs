<h1 align="center">
  <br />
  🪟
  <br />
  windows-resize-rs
  <sup>
    <br />
    <br />
  </sup>
</h1>

<div align="center">
  <sup>Node.js native module for restricting window resize direction (Windows only)</sup>
  <br />
  <br />
</div>

## Introduction

`windows-resize-rs` is a Node.js native module for restricting window resize direction on Windows.

## Installation

```bash
$ npm install windows-resize-rs
# or
$ yarn add windows-resize-rs
# or
$ pnpm add windows-resize-rs
```

## Usage

```js
const { BrowserWindow } = require('electron');
const { restrictResize } = require('windows-resize-rs');

const win = new BrowserWindow({
  width: 800,
  height: 600,
});

const handleBuffer = win.getNativeWindowHandle();
const handle = handleBuffer.readUInt32LE();

restrictResize(handle);
```