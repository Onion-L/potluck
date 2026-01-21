-- Composite index: Optimize queries filtering by source and sorting by date
-- Example: SELECT * FROM articles WHERE source = 'Smol AI' ORDER BY published_at DESC
CREATE INDEX IF NOT EXISTS idx_articles_published_source
  ON articles(published_at DESC, source);
