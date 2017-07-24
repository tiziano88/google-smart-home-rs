Enable ip forwarding:

```
sysctl -w net.ipv4.ip_forward=1
sysctl -w net.ipv6.conf.all.forwarding=1
```

```
cargo run
```

```
ngrok http 1234
```

## Setup instructions

1. Compile and run the server with `cargo run`.
1. Run `ngrok http 1234` and take note of the public HTTPS address, something like `https://xxxxxxxx.ngrok.io`.
1. Locally modify the `action.json` file, setting the `conversations.automation.url` field to `$URL/action`.

1. Using the [gActions CLI](https://developers.google.com/actions/tools/gactions-cli), run `gactions update --action_package=action.json` `--project=$PROJECT`.

1. Navigate to the [Actions Console](https://console.actions.google.com) and click Add/Import Project.
1. Create a new project or import an existing Google Cloud project.
1. Navigate to the app overview.
1. Under "App information" click "ADD":
    1. Under "Name":
        1. Fill in "Display Name".
        1. Fill in "Pronunciation".
        1. Click "NEXT".
    1. Under "Details": Fill in "Introduction", "Short Description", "Full Description", "Category", and click "NEXT".
    1. Under "Images": Upload both a large banner and a small banner and click "NEXT".
    1. Under "Testing instructions (optional)": Click "NEXT".
    1. Under "Contact Details": Fill in "Email" and click "NEXT".
    1. Under "Privacy and consent": Fill in "Link to Privacy Policy" and click "SAVE".
1. Under "Account linking (optional)" click "ADD":
    1. Under "Grant type":
        1. Select "Authorization code".
        1. Click "NEXT".
    1. Under "Client information":
        1. Fill in "Client ID" to any value.
        1. Fill in "Client secret" to any value.
        1. Fill in "Authorization URL" to `$URL/auth`.
        1. Fill in "Token URL" to `$URL/token`.
        1. Click "NEXT".
    1. Under "Configure your client (optional)":
        1. Click "NEXT".
    1. Under "Testing instructions":
        1. Fill in "Testing instructions".
    1. Click "SAVE".
