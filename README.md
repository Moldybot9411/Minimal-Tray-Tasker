# Minimal Tray Tasker

Track your everyday tasks right from your system tray

<img src="./images/Screenshot.png" alt="demo" width="300"/>

## Background
This was a little personal project to get a bit more familiar with the Tauri Framework.
I needed something that sometimes reminds me to stay on track and give me a simple overview of things I still have to do.

## Features
- Add Tasks: Tasks can be a one time thing or something you want to do daily. When a daily Task was finished, it will be reset at midnight.
- Reminders: Every full hour you'll get a notification to remind you, that you still have stuff to do. This can be turned off in the settings.
- Autostart: The program will be in your tray from the second your system starts. This can be turned off in the settings.

## Install

Head to the releases page and download the right package for your system.

## Developing

Install Node dependencies
```
npm i
```

Run the project
```
npm run tauri dev
```

Build the project
```
npm run tauri build
```
