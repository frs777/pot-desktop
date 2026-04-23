# GitHub Pages Setup Instructions

## Required Manual Step

Go to: **Settings → Pages** in this repository

Set:
- **Source**: Deploy from a branch
- **Branch**: `master` 
- **Folder**: `/docs`
- Click **Save**

After saving, the site will be available at:
**https://frs777.github.io/pot-desktop/**

## Alternative: Use GitHub Actions

If you prefer GitHub Actions deployment:

1. Go to **Settings → Pages**
2. Under **Build and deployment**, select **GitHub Actions**
3. The workflow `.github/workflows/deploy-pages.yml` will handle deployment automatically

## Files

- `index.html` - Polish version (default)
- `index-en.html` - English version
