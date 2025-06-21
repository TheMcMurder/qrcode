# GitHub Pages Deployment Guide

This guide explains how to deploy the QR Code Generator web example to GitHub Pages.

## Overview

The project is configured for automatic deployment to GitHub Pages using GitHub Actions. When you push to the `main` branch, the workflow will:

1. Build the WASM bindings
2. Build the web application
3. Deploy to GitHub Pages

## Prerequisites

1. **GitHub Repository**: Ensure your repository is named `qrcode` (or update the base path in `vite.config.ts`)
2. **GitHub Pages Enabled**: Enable GitHub Pages in your repository settings
3. **GitHub Actions Permissions**: Ensure GitHub Actions has permission to deploy to Pages

## Setup Steps

### 1. Enable GitHub Pages

1. Go to your repository on GitHub
2. Navigate to **Settings** → **Pages**
3. Under **Source**, select **GitHub Actions**
4. Save the settings

### 2. Configure Repository Settings

1. Go to **Settings** → **Actions** → **General**
2. Ensure **Actions permissions** is set to **Allow all actions and reusable workflows**
3. Under **Workflow permissions**, select **Read and write permissions**
4. Check **Allow GitHub Actions to create and approve pull requests**
5. Save the settings

### 3. Update Repository Name (if needed)

If your repository is not named `qrcode`, update the base path in `examples/web/vite.config.ts`:

```typescript
base: process.env.NODE_ENV === 'production' ? '/your-repo-name/' : '/',
```

### 4. Update Demo URLs

Update the demo URLs in the following files to match your GitHub username and repository name:

- `README.md` (line with live demo)
- `examples/web/README.md` (line with live demo)

Replace `yourusername` with your actual GitHub username.

## Deployment Process

### Automatic Deployment

The deployment happens automatically when you push to the `main` branch. The workflow:

1. **Checks out** the code
2. **Sets up** Node.js, Rust, and pnpm
3. **Installs** wasm-pack
4. **Builds** the WASM bindings
5. **Updates** the WASM dependency
6. **Builds** the web application
7. **Deploys** to GitHub Pages

### Manual Deployment

If you need to deploy manually:

1. Build the project:
   ```bash
   cd examples/web
   pnpm run build:gh-pages
   ```

2. The built files will be in `examples/web/dist/`

3. Upload the contents of `dist/` to your GitHub Pages branch

## Configuration Files

### GitHub Actions Workflow

The workflow is defined in `.github/workflows/deploy.yml` and includes:

- **Triggers**: Push to main, pull requests
- **Permissions**: Pages write, id-token write
- **Concurrency**: Prevents multiple deployments
- **Build Job**: Sets up environment and builds
- **Deploy Job**: Deploys to GitHub Pages

### Vite Configuration

The `examples/web/vite.config.ts` includes:

- **Base Path**: `/qrcode/` for production
- **Build Output**: Optimized for static hosting
- **WASM Handling**: Proper asset handling
- **Chunk Splitting**: Vendor and application chunks

### SPA Routing

The project includes SPA routing support:

- **404.html**: Redirects to main page
- **index.html**: Handles client-side routing
- **Base Path**: Correctly configured for GitHub Pages

## Troubleshooting

### Build Failures

1. **WASM Build Issues**:
   - Ensure Rust toolchain is up to date
   - Check wasm-pack installation
   - Verify WASM target is set to 'web'

2. **Dependency Issues**:
   - Clear node_modules and reinstall
   - Update pnpm lock file
   - Check for version conflicts

3. **TypeScript Errors**:
   - Run `pnpm run lint` to check for issues
   - Ensure all types are properly defined

### Deployment Issues

1. **404 Errors**:
   - Check base path configuration
   - Ensure 404.html is included in build
   - Verify SPA routing is working

2. **WASM Loading Issues**:
   - Check WASM file is included in build
   - Verify MIME types are correct
   - Test in different browsers

3. **GitHub Actions Failures**:
   - Check workflow logs for specific errors
   - Verify permissions are set correctly
   - Ensure all required tools are available

### Performance Issues

1. **Large Bundle Size**:
   - WASM file is ~1.7MB (normal for QR code library)
   - Consider code splitting for larger applications
   - Use compression in production

2. **Slow Loading**:
   - Enable GitHub Pages compression
   - Use CDN for static assets
   - Implement lazy loading

## Monitoring

### GitHub Actions

Monitor deployments in the **Actions** tab:

- **Workflow Runs**: View all deployment attempts
- **Logs**: Check for build and deployment errors
- **Artifacts**: Download build outputs for debugging

### GitHub Pages

Monitor the live site:

- **Site Health**: Check for 404s and errors
- **Performance**: Monitor loading times
- **Analytics**: Enable GitHub Pages analytics

## Security Considerations

1. **WASM Security**: WebAssembly runs in a sandbox
2. **Dependencies**: Regularly update dependencies
3. **Content Security Policy**: Consider adding CSP headers
4. **HTTPS**: GitHub Pages provides HTTPS by default

## Future Enhancements

1. **Custom Domain**: Configure custom domain for the demo
2. **Analytics**: Add analytics to track usage
3. **Performance Monitoring**: Implement performance monitoring
4. **A/B Testing**: Test different features with users

## Support

If you encounter issues:

1. Check the [GitHub Actions documentation](https://docs.github.com/en/actions)
2. Review the [GitHub Pages documentation](https://docs.github.com/en/pages)
3. Open an issue in the repository
4. Check the troubleshooting section above 