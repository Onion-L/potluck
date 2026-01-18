# Agent Guidelines for Potluck

This repository contains a Nuxt 4 application ("Potluck") using Vue 3, Nuxt UI, and Tailwind CSS.
These guidelines define how to interact with the codebase, run commands, and maintain code style.

## 1. Commands

Use `pnpm` for all package management and script execution.

| Action | Command | Description |
| :--- | :--- | :--- |
| **Install** | `pnpm install` | Install dependencies. |
| **Dev Server** | `pnpm dev` | Start development server at http://localhost:3000. |
| **Build** | `pnpm build` | Build the application for production. |
| **Lint** | `pnpm lint` | Run ESLint across the project. |
| **Typecheck** | `pnpm typecheck` | Run Vue/TypeScript type checking. |
| **Preview** | `pnpm preview` | Preview the production build locally. |

**Testing Note:**
There is currently no dedicated test runner (like Vitest) configured.
To verify changes, **always run `pnpm typecheck` and `pnpm lint`**.
If you are adding a significant feature, consider proposing the addition of Vitest.

## 2. Tech Stack

- **Framework:** Nuxt 4 (Vue 3)
- **UI Library:** @nuxt/ui v4
- **Styling:** Tailwind CSS v4
- **Language:** TypeScript
- **Package Manager:** pnpm

## 3. Code Style & Conventions

### General
- **Indentation:** 2 spaces.
- **Semicolons:** No semicolons.
- **Quotes:** Single quotes preferred.
- **Trailing Commas:** Never (enforced by ESLint config).

### Vue / Nuxt
- **Component Structure:**
  ```vue
  <script setup lang="ts">
  // Logic here
  </script>

  <template>
  <!-- Markup here -->
  </template>

  <style scoped>
  /* Styles here (rarely used, prefer Tailwind) */
  </style>
  ```
- **Auto-Imports:** Nuxt auto-imports Vue composables (`ref`, `computed`, `watch`) and Nuxt composables (`useFetch`, `useRouter`, `useColorMode`). **Do not manually import these.**
- **Components:** Use PascalCase for component names (e.g., `AppHeader.vue`).
- **Icons:** Use the `UIcon` component with the `name` prop.
  - Format: `i-{collection}-{name}` (e.g., `i-lucide-loader-2`, `i-heroicons-moon-20-solid`).
  - Prefer `lucide` or `heroicons` collections.

### TypeScript
- Use strict typing. Avoid `any`.
- Define interfaces/types locally in the component if they are specific to it, or in `types/` if shared.
- Use `interface` over `type` for object definitions where possible.

### Tailwind / Styling
- Use utility classes directly in the template.
- Use `class` attribute (not `className`).
- Support Dark Mode: Use `dark:` variants (e.g., `bg-stone-50 dark:bg-stone-950`).
- Color Palette: The project seems to heavily use `stone` for neutrals. Adhere to this palette.

## 4. Project Structure

- `app/`: Main application source code.
  - `components/`: Vue components (auto-imported).
  - `pages/`: Application routes (file-system routing).
  - `assets/`: Static assets and global CSS (`css/main.css`).
  - `app.config.ts`: App-level configuration.
- `server/`: Server-side logic (Nitro).
- `public/`: Public static files (e.g., `data/latest.json`, images).
- `nuxt.config.ts`: Main Nuxt configuration.

## 5. Error Handling
- For data fetching, handle `pending` and `error` states returned by `useFetch`.
- Use `<ClientOnly>` for components that rely heavily on browser APIs or might cause hydration mismatches (like date formatting dependent on locale).

## 6. Git Commit Convention
- Use conventional commits (e.g., `feat:`, `fix:`, `chore:`, `docs:`, `style:`, `refactor:`).
- Keep messages concise and descriptive (imperative mood).
