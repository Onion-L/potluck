# Nuxt Starter Template

[![Nuxt UI](https://img.shields.io/badge/Made%20with-Nuxt%20UI-00DC82?logo=nuxt&labelColor=020420)](https://ui.nuxt.com)

Use this template to get started with [Nuxt UI](https://ui.nuxt.com) quickly.

- [Live demo](https://starter-template.nuxt.dev/)
- [Documentation](https://ui.nuxt.com/docs/getting-started/installation/nuxt)

<a href="https://starter-template.nuxt.dev/" target="_blank">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://ui.nuxt.com/assets/templates/nuxt/starter-dark.png">
    <source media="(prefers-color-scheme: light)" srcset="https://ui.nuxt.com/assets/templates/nuxt/starter-light.png">
    <img alt="Nuxt Starter Template" src="https://ui.nuxt.com/assets/templates/nuxt/starter-light.png">
  </picture>
</a>

> The starter template for Vue is on https://github.com/nuxt-ui-templates/starter-vue.

## Quick Start

```bash [Terminal]
npm create nuxt@latest -- -t github:nuxt-ui-templates/starter
```

## Deploy your own

[![Deploy with Vercel](https://vercel.com/button)](https://vercel.com/new/clone?repository-name=starter&repository-url=https%3A%2F%2Fgithub.com%2Fnuxt-ui-templates%2Fstarter&demo-image=https%3A%2F%2Fui.nuxt.com%2Fassets%2Ftemplates%2Fnuxt%2Fstarter-dark.png&demo-url=https%3A%2F%2Fstarter-template.nuxt.dev%2F&demo-title=Nuxt%20Starter%20Template&demo-description=A%20minimal%20template%20to%20get%20started%20with%20Nuxt%20UI.)

## Setup

Make sure to install the dependencies:

```bash
pnpm install
```

## Local Development

This project uses Supabase for local development with Docker.

**Start local Supabase:**

```bash
supabase start
```

This will start all Supabase services (database, auth, storage) locally.

**Run the development server:**

```bash
pnpm dev
```

The app will be available at `http://localhost:3000`.

**Migrations:**

Create manual migration files in `supabase/migrations/`. Migrations are applied automatically when starting local Supabase.

**Environment:**

Switch between environments using `.env.local` for local development overrides.

## Development Server

Start the development server on `http://localhost:3000`:

```bash
pnpm dev
```

## Production

Build the application for production:

```bash
pnpm build
```

Locally preview production build:

```bash
pnpm preview
```

Check out the [deployment documentation](https://nuxt.com/docs/getting-started/deployment) for more information.

## 🛠️ Data Flow & API Guide

### Data Flow
`RSS Feeds` → `/api/ingest` → `Supabase (articles table)` → `/api/latest` → `Frontend`

### Local Setup (Detailed)
1. **Start Database**:
   ```bash
   pnpm supabase start
   ```
   *Note: Ensure you copy the output Keys to `.env.local`*

2. **Configure Environment**:
   ```bash
   cp .env.example .env.local
   # Set SUPABASE_URL, SUPABASE_KEY, SUPABASE_SERVICE_KEY, INGEST_API_KEY
   ```

3. **Start App**:
   ```bash
   pnpm dev
   ```

### API Reference

| Endpoint | Method | Auth | Description |
|----------|--------|------|-------------|
| `/api/latest` | GET | None | Fetch latest 50 articles. Publicly accessible. |
| `/api/ingest` | POST | Bearer Token | Trigger RSS fetch & AI processing. Requires `INGEST_API_KEY`. |

**Manual Ingest Trigger:**
```bash
curl -X POST http://localhost:3000/api/ingest \
  -H "Authorization: Bearer <your-ingest-key>"
```
