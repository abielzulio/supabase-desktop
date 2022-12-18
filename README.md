<p align="center">
<img width=100 src="src-tauri/icons/icon.png">
</p>

<h1 align="center">Supabase Desktop App</h1>

<h3 align="center">
Web-wrapped Supabase desktop app running on macOS, Windows, and Linux powered by Tauri
</h3>

![Individual user project screen](https://user-images.githubusercontent.com/7030944/208304068-f71b14f4-4d18-4134-b648-3eb9aae2f8c6.png)

## F.A.Q

### Why do I need this? Why not just open it regularly via the browser?

Supabase is one of the essential dev apps that I need to open it daily. Opening it straight from the dock/app tray is just a lot quicker and easier than having to click the browser icon, type the URL or click the bookmark item. It's just more convenient.

![User projects screen](https://user-images.githubusercontent.com/7030944/208304046-65b29f9d-b455-495b-84a2-38a06800b25e.png)

### Is it safe to login in your app? Do you store my login credentials?

Yes, it is totally safe. No, I don't store your login credentials as I have no control. You can check the source code to make sure there is no data transferred between me and you.

![User login screen](https://user-images.githubusercontent.com/7030944/208304004-6b0623a4-d88f-4c83-a9b8-bc67b354fe27.png)

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
