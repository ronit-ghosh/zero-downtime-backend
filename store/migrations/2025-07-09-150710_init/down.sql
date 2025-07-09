-- Drop foreign key constraints first
ALTER TABLE "website_tick" DROP CONSTRAINT IF EXISTS "website_tick_region_id_fkey";
ALTER TABLE "website_tick" DROP CONSTRAINT IF EXISTS "website_tick_website_id_fkey";

-- Drop tables in reverse order (child tables first)
DROP TABLE IF EXISTS "website_tick";
DROP TABLE IF EXISTS "region";
DROP TABLE IF EXISTS "website";
DROP TABLE IF EXISTS "user";

-- Drop enum type
DROP TYPE IF EXISTS "website_status";
