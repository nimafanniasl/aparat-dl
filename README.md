![Screenshot from 2024-02-02 23-03-15](https://github.com/nimafanniasl/aparat-dl/assets/76901932/5af2b4b8-8d09-4563-b32e-1591c839c824)

### Simple command line tool to download videos from Aparat!
### Written in 🦀 as a fun side-project to learn more about this amazing language :)

### Usage:
For now, this project doesn't have any installers, but I provide pre-built GNU/Linux binaries (X86_64) for you to download and run.

Use wget to get the binary:

```bash
wget https://github.com/nimafanniasl/aparat-dl/raw/main/Binaries/aparat-dl
```

Then run it:

```bash
./aparat-dl
```

Example:

```bash
./aparat-dl -l https://www.aparat.com/v/D9Emy -q 1080p -s /home/user/Downloads
```

Also, you can move it to your PATH and run it from anywhere. but I'm working on an installer to do that automatically :)

### TODO:
- [x] Downloads files correctly
- [ ] Provide pre-built binaries for Windows and macOS
- [x] Get file save path as an optional arg from the user
- [ ] Show a progress bar for downloading the file
- [x] Download playlists
- [ ] Make an installer for the project.
- [ ] Use Github actions for automating releases
- [ ] Define the JSON data from the api with a struct/enum as [Marc Planard suggested here.](https://mstdn.social/@corpsmoderne@mamot.fr/111867224367431809)
