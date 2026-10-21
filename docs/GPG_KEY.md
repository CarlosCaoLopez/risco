# Signing commits with GPG

Signed commits prove that a commit really comes from the person named in it.
GitHub marks them as **Verified**, and the `main` and `develop` branches of
Risco require them. This guide takes about ten minutes.

## 1. Generate a key pair

Install GnuPG if you do not have it (`sudo apt install gnupg`,
`brew install gnupg`, or [Gpg4win](https://www.gpg4win.org/) on Windows), then:

```sh
gpg --full-generate-key
```

Answer the prompts:

- **Kind of key**: `ECC (sign and encrypt)` with `Curve 25519`, or `RSA and RSA` with 4096 bits.
- **Expiration**: `2y` is a good default. You can extend it later.
- **Name and email**: use the **same email** as your Git commits
  (`git config user.email`), and make sure it is verified on your GitHub account.
- **Passphrase**: choose a strong one. It protects the key if your disk is stolen.

## 2. Find your key ID

```sh
gpg --list-secret-keys --keyid-format=long
```

```
sec   ed25519/3AA5C34371567BD2 2026-10-21 [SC] [expires: 2028-10-21]
      28283B5F24504E9D3BC9040E3AA5C34371567BD2
uid                 [ultimate] Your Name <you@example.com>
```

The key ID is the part after the slash: `3AA5C34371567BD2`.

## 3. Configure Git

```sh
git config --global user.signingkey 3AA5C34371567BD2
git config --global commit.gpgsign true
git config --global tag.gpgsign true
```

From now on every commit and tag is signed. If GPG cannot ask for your
passphrase in a terminal, add this to your shell profile:

```sh
export GPG_TTY=$(tty)
```

## 4. Export and publish the public key

```sh
gpg --armor --export 3AA5C34371567BD2
```

Copy everything from `-----BEGIN PGP PUBLIC KEY BLOCK-----` to
`-----END PGP PUBLIC KEY BLOCK-----`, then on GitHub go to
**Settings › SSH and GPG keys › New GPG key** and paste it.

Optionally publish it to a keyserver so others can verify your signatures
outside GitHub:

```sh
gpg --keyserver keys.openpgp.org --send-keys 3AA5C34371567BD2
```

## 5. Verify signed commits

```sh
git log --show-signature -3
```

A good signature looks like:

```
commit 6744966…
gpg: Signature made Tue 21 Oct 2026 18:02:11 CEST
gpg:                using EDDSA key 28283B5F24504E9D3BC9040E3AA5C34371567BD2
gpg: Good signature from "Your Name <you@example.com>" [ultimate]
```

`git log --format='%h %G? %s'` shows one letter per commit: `G` good,
`N` no signature, `B` bad, `E` cannot be checked (missing public key).

To check a release tag: `git tag -v v0.1.1`.

## 6. Branch protection on GitHub

Maintainers configure this once, in **Settings › Branches › Add branch
protection rule** (or **Rules › Rulesets**), for both `main` and `develop`:

- [x] Require a pull request before merging, with **1** required approval
- [x] Require status checks to pass: `build`, `lint`, `test`
- [x] **Require signed commits**
- [x] Do not allow bypassing the above settings
- [ ] Allow force pushes (leave off)
- [ ] Allow deletions (leave off)

## Troubleshooting

| Problem | Fix |
|:--|:--|
| `error: gpg failed to sign the data` | Run `echo test \| gpg --clearsign` to see the real error. Usually `GPG_TTY` is unset or the pinentry window timed out. |
| `signing failed: Timeout` / `Vida máxima` | The passphrase prompt expired. Run the commit again and type the passphrase promptly, or raise `default-cache-ttl` in `~/.gnupg/gpg-agent.conf`. |
| GitHub shows **Unverified** | The commit email is not a verified email on your GitHub account, or the key there is a different one. |
| Key expired | `gpg --edit-key <ID>`, then `expire`, then `save`, and upload the public key to GitHub again. |
