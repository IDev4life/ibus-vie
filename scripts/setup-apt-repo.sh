#!/usr/bin/env bash
# One-time setup: generate GPG key, init gh-pages branch, print GitHub secrets.
# Run from repo root: bash scripts/setup-apt-repo.sh
set -euo pipefail

REPO_URL=$(git remote get-url origin)
EMAIL=$(git config user.email)
NAME=$(git config user.name)

echo "=== ibus-vie apt repository setup ==="
echo "Repo : $REPO_URL"
echo "Name : $NAME <$EMAIL>"
echo ""

# 1. Generate GPG key if needed
if ! gpg --list-secret-keys "$EMAIL" &>/dev/null; then
    echo "[1/4] Generating GPG key for $EMAIL..."
    gpg --batch --gen-key <<EOF
Key-Type: RSA
Key-Length: 4096
Subkey-Type: RSA
Subkey-Length: 4096
Name-Real: $NAME
Name-Email: $EMAIL
Expire-Date: 0
%no-protection
%commit
EOF
else
    echo "[1/4] GPG key for $EMAIL already exists."
fi

GPG_ID=$(gpg --list-secret-keys --with-colons "$EMAIL" | awk -F: '/^sec/{print $5; exit}')
echo "      Key ID: $GPG_ID"

# 2. Init gh-pages branch
echo ""
echo "[2/4] Initializing gh-pages branch..."
if git ls-remote --exit-code origin gh-pages &>/dev/null; then
    echo "      gh-pages already exists on remote."
else
    git checkout --orphan gh-pages
    git rm -rf . --quiet
    mkdir -p apt
    echo "# ibus-vie apt repository" > README.md
    git add README.md apt
    git commit -m "init: empty apt repository"
    git push origin gh-pages
    git checkout main
    echo "      gh-pages branch created and pushed."
fi

# 3. Export keys
echo ""
echo "[3/4] Exporting keys..."
GPG_PRIVATE_KEY=$(gpg --armor --export-secret-keys "$EMAIL")
echo ""
echo "=== ADD THESE TO GITHUB SECRETS (Settings → Secrets → Actions) ==="
echo ""
echo "Secret name: GPG_PRIVATE_KEY"
echo "Secret value:"
echo "$GPG_PRIVATE_KEY"
echo ""
echo "=== ALSO ENABLE GITHUB PAGES ==="
echo "  Settings → Pages → Source: Deploy from branch → Branch: gh-pages / (root)"
echo ""
echo "[4/4] Done. After adding secrets and enabling Pages, push a tag to trigger release:"
echo "  git tag v1.1.0 && git push origin v1.1.0"
echo ""
echo "User install command:"
echo "  curl -fsSL https://$(echo $REPO_URL | sed 's/.*github.com[:/]\(.*\)\.git/\1/' | tr '/' '.').github.io/ibus-vie/KEY.gpg \\"
echo "    | sudo gpg --dearmor -o /etc/apt/keyrings/ibus-vie.gpg"
