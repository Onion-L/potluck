# Agent Guidelines for Potluck

This repository contains a **Nuxt 4** application ("Potluck") using **Vue 3**, **Nuxt UI v4**, and **Tailwind CSS v4**.
These guidelines define how to interact with the codebase, run commands, and maintain code style.

## 1. Commands

Use `pnpm` for all package management and script execution.

| Action | Command | Description |
| :--- | :--- | :--- |
| **Install** | `pnpm install` | Install dependencies. |
| **Dev Server** | `pnpm dev` | Start development server at http://localhost:3000. |
| **Build** | `pnpm build` | Build the application for production (Nitro + Vue). |
| **Lint** | `pnpm lint` | Run ESLint across the project (fix with `--fix`). |
| **Typecheck** | `pnpm typecheck` | Run Vue/TypeScript type checking. |
| **Preview** | `pnpm preview` | Preview the production build locally. |
| **Supabase** | `pnpm supabase` | Run local Supabase CLI commands. |

### Testing Strategy
There is currently **no dedicated test runner** (like Vitest) configured.
*   **Verification:** To verify changes, you **MUST** run `pnpm typecheck` and `pnpm lint`.
*   **New Features:** If you are adding a significant feature, consider proposing the addition of Vitest.

## 2. Tech Stack

*   **Framework:** Nuxt 4 (Vue 3 Composition API)
*   **UI Library:** @nuxt/ui v4 (Alpha/Beta - check docs for v4 specifics)
*   **Styling:** Tailwind CSS v4 (No `tailwind.config.js`, uses CSS-first config)
*   **Language:** TypeScript (Strict mode)
*   **Backend:** Nitro (Server routes), Supabase (Database/Auth)
*   **Package Manager:** pnpm

## 3. Code Style & Conventions

### General Formatting
*   **Indentation:** 2 spaces.
*   **Semicolons:** **None** (Strictly enforced).
*   **Quotes:** Single quotes preferred.
*   **Trailing Commas:** **Never** (Enforced by ESLint config in `nuxt.config.ts`).
*   **Brace Style:** `1tbs` (One True Brace Style).

### Vue / Nuxt Patterns
*   **Component Structure:**
    ```vue
    <script setup lang="ts">
    // 1. Props/Emits definitions
    // 2. Composables (useRouter, useFetch)
    // 3. Computed properties
    // 4. Methods/Functions
    </script>

    <template>
      <!-- Root element if possible (Vue 3 allows fragments, but single root is safer for transitions) -->
      <div class="p-4">
        <!-- Markup -->
      </div>
    </template>
    ```
*   **Auto-Imports:** Nuxt auto-imports Vue composables (`ref`, `computed`, `watch`) and Nuxt composables (`useFetch`, `useRouter`, `useColorMode`).
    *   **Rule:** Do NOT manually import these.
*   **Component Naming:** PascalCase for filenames and usage (e.g., `AppHeader.vue` -> `<AppHeader />`).
    *   Base components should often be prefixed if they are project-specific atoms.
*   **Icons:** Use the `UIcon` component with the `name` prop.
    *   Format: `i-{collection}-{name}` (e.g., `i-lucide-loader-2`).
    *   **Preferred Collection:** `lucide` or `heroicons`.

### TypeScript Rules
*   **Strict Mode:** Always enabled. Avoid `any` at all costs.
*   **Interfaces:** Use `interface` for object shapes, `type` for unions/primitives.
*   **Explicit Returns:** Prefer explicit return types for complex functions, especially in `server/` routes.
*   **DTOs:** Define shared data types in a `types/` directory or collocated if private.

### Tailwind / Styling
*   **Config:** Tailwind v4 uses CSS detection. Do not look for a config file.
*   **Classes:** Use utility classes directly in the template `class` attribute.
*   **Dark Mode:** Support is mandatory. Use `dark:` variants (e.g., `bg-stone-50 dark:bg-stone-950`).
*   **Color Palette:**
    *   **Primary/Neutral:** `stone` (configured in `app.config.ts`).
    *   Avoid hardcoding hex values; use Tailwind semantic colors.

## 4. Architecture & Structure

*   **`app/`**: Main application source code.
    *   `components/`: Vue components (auto-imported).
    *   `pages/`: Application routes (file-system routing).
    *   `assets/`: Static assets and global CSS (`css/main.css`).
    *   `app.config.ts`: App-level UI configuration.
*   **`server/`**: Nitro server-side logic.
    *   `api/`: API routes (e.g., `/api/latest`).
    *   `utils/`: Server-side utilities.
    *   **Rule:** Database access (Supabase Admin) happens here.
*   **`nuxt.config.ts`**: Main configuration.
    *   **Runtime Config:** Use `runtimeConfig` for secrets (server-only) and `runtimeConfig.public` for client-exposed vars.

## 5. Data & Supabase

*   **Client-Side:** Use `useSupabaseClient()` inside Vue components.
*   **Server-Side:** Use `serverSupabaseClient(event)` inside Nitro event handlers.
*   **Auth:** `redirect: false` is configured. Handle auth redirects manually or via middleware if needed.
*   **Fetching:**
    *   Use `useFetch` for data fetching in components (SSR-friendly).
    *   Handle `pending`, `error`, and `data` states explicitly.
    *   Example:
        ```typescript
        const { data, status, error } = await useFetch('/api/latest')
        ```

## 6. Environment Variables

*   **File:** `.env` (local development).
*   **Secrets:** Never commit `.env` files.
*   **Access:**
    *   **Client:** `useRuntimeConfig().public.keyName`
    *   **Server:** `useRuntimeConfig().keyName` or `process.env.KEY_NAME` (in config).

## 7. Git Commit Convention

Follow the **Conventional Commits** specification:

*   `feat: description` - New feature
*   `fix: description` - Bug fix
*   `docs: description` - Documentation
*   `style: description` - Formatting, missing semi colons, etc; no code change
*   `refactor: description` - Refactoring production code
*   `chore: description` - Build process, aux tools, libraries

**Message Style:**
*   Imperative mood ("Add feature" not "Added feature").
*   No period at the end.
*   Keep it concise (under 72 chars ideally).
