# Vercel Deployment Guide for MoveForge

## ✅ Fixed Issues:

1. **Removed WalletConnect dependencies** - They were causing npm registry errors
2. **Cleaned up unused UI components** - Avatar, Dialog, Dropdown, Navigation, Form, Sonner
3. **Minimal dependencies** - Only essential packages remain
4. **Proper Vercel configuration** - vercel.json routes to frontend subdirectory

## 📦 Current Dependencies:

```json
{
  "@radix-ui/react-label": "^2.1.8",
  "@radix-ui/react-slot": "^1.2.4",
  "class-variance-authority": "^0.7.1",
  "clsx": "^2.1.1",
  "lucide-react": "^0.553.0",
  "next": "16.0.7",
  "react": "19.2.0",
  "react-dom": "19.2.0",
  "tailwind-merge": "^3.4.0"
}
```

## 🚀 Vercel Deployment Steps:

### Option 1: Vercel CLI
```bash
cd "/home/marvi/Documents/M1 Hack"
vercel --prod
```

### Option 2: GitHub Integration (Recommended)

1. **Push to GitHub:**
```bash
git add .
git commit -m "fix: Remove WalletConnect deps for Vercel deployment"
git push origin main
```

2. **Import on Vercel:**
- Go to https://vercel.com/new
- Import your GitHub repository
- **Root Directory:** Leave empty (vercel.json handles routing)
- **Framework Preset:** Next.js
- **Build Command:** (auto-detected)
- **Output Directory:** (auto-detected)
- Deploy!

### Option 3: Manual Configuration

If auto-detection fails, set:
- **Root Directory:** `frontend`
- **Build Command:** `npm run build`  
- **Output Directory:** `.next`
- **Install Command:** `npm install --legacy-peer-deps`

## ✅ Build Verification:

Test locally before deploying:
```bash
cd frontend
npm run build
npm run start
```

Visit http://localhost:3000 - should work perfectly!

## 🎯 Expected Vercel Build Output:

```
✓ Compiled successfully
✓ Generating static pages (6/6)
✓ Finalizing page optimization

Routes:
  ○ /              (Static)
  ○ /dashboard     (Static)
  ○ /docs          (Static)
```

## 🔍 Troubleshooting:

If build fails on Vercel:

1. **Check build logs** for specific error
2. **Verify Node version** - Should be 18.x or 20.x
3. **Clear build cache** in Vercel project settings
4. **Retry deployment** - Sometimes npm registry has temporary issues

## 📊 What Changed:

- ❌ Removed: `@reown/appkit`, `@reown/appkit-adapter-wagmi`, `wagmi`, `viem`, `@tanstack/react-query`
- ❌ Removed: `context/`, `config/`, `.env.local` directories
- ❌ Removed: Unused UI components (avatar, dialog, dropdown, navigation, form)
- ✅ Added: Minimal radix-ui dependencies only for used components
- ✅ Added: vercel.json configuration
- ✅ Added: .vercelignore to exclude Rust files

The dashboard now works in Demo Mode - no wallet required!
