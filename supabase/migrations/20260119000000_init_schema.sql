-- Initial schema for Potluck RSS reader

-- Create feeds table
create table if not exists feeds (
  id uuid primary key default gen_random_uuid(),
  url text not null unique,
  name text not null,
  is_active boolean default true,
  created_at timestamptz default now()
);

-- Create articles table
create table if not exists articles (
  id uuid primary key default gen_random_uuid(),
  feed_id uuid references feeds(id) on delete cascade,
  title text not null,
  url text not null unique,
  summary text,
  tags text[],
  source text,
  published_at timestamptz not null,
  created_at timestamptz default now()
);

-- Create index for ordering articles by published date
create index if not exists articles_published_at_idx on articles(published_at desc);

-- Create index for feed lookups
create index if not exists articles_feed_id_idx on articles(feed_id);
