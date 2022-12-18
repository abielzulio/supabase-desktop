<h1 align="center">Supabase Desktop App</h1>

Web-wrapped Supabase desktop app running on MacOS, Windows, and Linux using Tauri

## F.A.Q

### Why do I need it? Why not just open it reguraly via the browser?

Supabase is one of the essential dev apps that I need to open it daily. Opening it straight from the dock/app tray is just a lot quicker and easier than having to click the browser icon, type the URL or click the bookmark item. It's just more convenient.

### Is it safe to login in your app? Do you store my login credentials?

Yes, it is totally safe. I have no control over your authentication credentials. You can check the source code to make sure there is no data transferred between me and you.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Developing

```
npm install
npm run tauri dev
```

### Building

```
npm install
npm run tauri build
```
