-- =============================================
-- RLS Policies for Articles Table
-- =============================================

-- Enable RLS on articles table
alter table articles enable row level security;

-- Allow public read access (for the frontend)
create policy "Public Read Articles"
  on articles for select
  using ( true );

-- Allow authenticated insert (for ingest script with Service Role or Auth User)
-- Note: Service Role bypasses RLS, so this is mainly for potential future authenticated users
create policy "Authenticated Insert Articles"
  on articles for insert
  with check ( auth.role() = 'authenticated' );

-- =============================================
-- RLS Policies for Feeds Table
-- =============================================

-- Enable RLS on feeds table
alter table feeds enable row level security;

-- Allow public read access (feeds list is not sensitive)
create policy "Public Read Feeds"
  on feeds for select
  using ( true );

-- Only allow authenticated users to modify feeds
-- Note: Service Role bypasses RLS for admin operations
create policy "Authenticated Modify Feeds"
  on feeds for all
  using ( auth.role() = 'authenticated' )
  with check ( auth.role() = 'authenticated' );
