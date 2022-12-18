<p align="center">
<img width=100 src="src-tauri/icons/icon.png">
</p>

<h1 align="center">Supabase Desktop App</h1>

![Individual user project screen](https://user-images.githubusercontent.com/7030944/208304068-f71b14f4-4d18-4134-b648-3eb9aae2f8c6.png)


## What is it?

It's a cross-platform web-wrapped Supabase desktop app powered by Tauri. You can install it on your macOS, Windows (untested), or Linux (untested).

![User projects screen](https://user-images.githubusercontent.com/7030944/208304046-65b29f9d-b455-495b-84a2-38a06800b25e.png)

![User login screen](https://user-images.githubusercontent.com/7030944/208304004-6b0623a4-d88f-4c83-a9b8-bc67b354fe27.png)

## F.A.Q

### Why do I need this? Why not just open it regularly via the browser?

Supabase is one of the essential dev apps that I need to open it daily. Opening it straight from the dock/app tray is just a lot quicker and easier than having to click the browser icon, type the URL or click the bookmark item. It's just more convenient.

### Is it safe to login in your app? Do you store my login credentials?

Yes, it is totally safe. No, I don't store your login credentials as I have no control. You can check the source code to make sure there is no data transferred between me and you.


### I can't install. What's happening?

![Warning pop-up](https://user-images.githubusercontent.com/7030944/208308948-6e85409a-d3e8-46cb-b1fe-ca9adb463f9e.png)


1. Close the warning pop-up by clicking the `Cancel` button
1. Go to `System Settings > Privacy & Security`
2. Click `Open Anyway` in the Supabase warning section

![Resolve step-by-step](https://user-images.githubusercontent.com/7030944/208309192-1c370c21-c45b-4004-ba4b-bf472427ce23.png)


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
