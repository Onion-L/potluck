# Daily Briefing & Timeline Refactor Design
**Date:** 2026-01-17
**Goal:** Split the single-page application into a focused "Daily Briefing" (Home) and a historical "Timeline".

## 1. Architecture

### Pages
- **`app/pages/timeline.vue`**:
  - **Purpose**: Displays the full history of news items.
  - **Implementation**: Clone of the original `index.vue`.
  - **Logic**: Fetches all data, groups by date, displays vertical timeline.
  - **Navigation**: Header link to "Today" (Home).

- **`app/pages/index.vue`**:
  - **Purpose**: "Daily Briefing" focused on today's news (`2026-01-17`).
  - **Implementation**:
    - Fetches data.
    - Filters for `publishedAt.startsWith('2026-01-17')`.
    - **Layout**: "Minimalist Magazine".
      - **Hero**: First article, emphasized typography (`text-5xl+`).
      - **List**: Remaining articles, clean vertical list.
    - **Empty State**: Graceful message if no news for "today".
  - **Navigation**: 
    - Header link to "History" (`/timeline`).
    - Footer button "View Full Timeline →".

### Components
- **`AppHeader` (New)**:
  - Extracting the header from `index.vue` to `app/components/AppHeader.vue` is recommended to ensure the Dark Mode toggle and Navigation Links (`Home` vs `Timeline`) are consistent.
  - **Props**: None.
  - **State**: Dark mode toggle.

## 2. Visual Design (Textured Minimalist)

- **Palette**: Stone-50/950 backgrounds, Orange-500 accents.
- **Typography**: Serif for headings (Hero), Sans for UI elements.
- **Texture**: `bg-noise` class preserved on both pages.
- **Hero**: High impact, large serif font, generous whitespace.
- **List**: Subtle separators, legible typography.

## 3. Implementation Steps

1.  **Refactor Header**: Extract existing header code to `app/components/AppHeader.vue`.
2.  **Create Timeline**: Copy current `index.vue` to `app/pages/timeline.vue` and update to use `AppHeader`.
3.  **Create Daily View**: Rewrite `app/pages/index.vue`:
    - Filter logic.
    - Hero + List HTML structure.
    - "View Full Timeline" CTA.
4.  **Verify**: Check both routes, dark mode, and responsiveness.
