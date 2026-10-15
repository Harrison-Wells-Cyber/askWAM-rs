# askWAM

A Windows command-line tool that requests Microsoft Entra access tokens through Windows Web Account Manager (WAM). It uses accounts already signed in to Windows and does not open a sign-in prompt. Results are printed as JSON.

## Build

On Windows, install [Rust](https://rustup.rs/) and Visual Studio C++ Build Tools. From the repo folder, run in PowerShell:

```powershell
.\build.ps1
```

This creates `bin\Release\askwam.exe`.

## Run

Request a token using the default Microsoft Teams client ID and Microsoft Graph scope:

```powershell
.\bin\Release\askwam.exe
```

List accounts, choose an account, or see all options:

```powershell
.\bin\Release\askwam.exe --enum
.\bin\Release\askwam.exe --username "you@example.com"
.\bin\Release\askwam.exe --help
```

Use `--client-id "APP_ID"` and `--scope "SCOPE"` to request a token for another app or API. Add `--hide` to leave the token out of the output.
