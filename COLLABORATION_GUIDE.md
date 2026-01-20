# Real-time Collaboration Guide

Latte now supports real-time collaboration using [Yjs](https://github.com/yjs/yjs) and WebSockets.

## How it works

1.  **Shared Document**: The editor state is stored in a shared Yjs document (`Y.Doc`).
2.  **Providers**: We use `y-websocket` to sync this document between clients via a central signaling server.
3.  **Bindings**:
    *   **Monaco (Code Mode)**: `y-monaco` binds the code text to the Yjs document.
    *   **TipTap (Rich Mode)**: `extension-collaboration` binds the rich text nodes to the Yjs document.

## Usage

1.  Open the **Settings** view in the Activity Bar (bottom icon).
2.  Scroll to the **Collaboration** section.
3.  Enter a **Server URL** (defaults to `wss://demos.yjs.dev` for testing).
4.  Enter a **Room Name** (e.g., `my-project`).
5.  Enter your **Username**.
6.  Click **Connect**.

Once connected, you will see other users' cursors and edits in real-time.

## Limitations (Current Implementation)

*   **Mode Synchronization**: 
    *   Collaboration works best when all users are in the same mode (all in Code or all in Rich Text).
    *   The document has two separate shared fields: `monaco` (text) and `document` (rich text XML).
    *   Switching modes locally performs a one-time conversion, which might overwrite the other field if not careful.
    *   *Best Practice*: Agree on a mode with your collaborators.

*   **Security**: The demo server (`wss://demos.yjs.dev`) is public. **Do not use it for sensitive data.** For production, you should host your own `y-websocket` server.

## Hosting a Server

To host your own collaboration server:

```bash
HOST=0.0.0.0 PORT=1234 npx y-websocket
```

Then connect using `ws://your-ip:1234`.
