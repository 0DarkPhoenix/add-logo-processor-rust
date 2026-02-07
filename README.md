# Add Logo Processor

Application to resize and add a logo to images in bulk. This project has been written before in Python, with this version being rewritten with a Rust back-end a Typescript front-end using Tauri

### Frontend
Typescript
Next.js + [Shadcn/ui](https://ui.shadcn.com/docs/installation)

### Updater through Tauri
https://v2.tauri.app/plugin/updater/

## Checklist for first Alpha release:
- [ ] Create better structured and more complete Readme.md
- [ ] Different options for resolution configuration modes
- [ ] Feedback in frontend for errors that happened in the backend
- [ ] Fix inconsistencies in frontend and backend when cancelling a process
- [ ] Set up Tauri updater and test config.json updater
- [ ] Set up Github actions for building application to Windows, MacOS and Linux after merge to main